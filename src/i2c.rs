//! Initialization and HAL implementation of the I2C peripheral
//!
//! The original I2C register block is transformed via a series of method invocations into a
//! ConfiguredI2C. The methods inbetween partially configure the device, and partially only pass on
//! information to a later call which configures it. That way was chosen because it allows the
//! built-in checks ("Does the selected route match the pins passed along?") to be eliminated at
//! build time.
//!
//! The exact invocation depends on the device series, as the routing mechanisms were changed. On
//! EFR32, a device is built like `i2c0.with_clock(cmu.i2c0).with_scl(LOC15,
//! pc11).unwrap().with_sda(LOC15, pc10).unwrap()`. On older EFM32, the `with_s{cl,da}` methods
//! would be replaced with a `with_route` that takes a single route designation and both pins at
//! once.

use embedded_hal;

use registers;

use gpio::{EFM32Pin, pins, Disabled};
use super::cmu;

pub trait I2CExt<Clk, WithClock> {
    fn with_clock(self, clock: Clk) -> WithClock;
}

impl I2CExt<cmu::I2C0Clk, I2C0WithClock> for registers::I2C0 {
    fn with_clock(self, mut clock: cmu::I2C0Clk) -> I2C0WithClock {
        clock.enable();
        self.clkdiv.write(|w| unsafe { w.div().bits(20) });

        self.ctrl.write(|w| w.en().bit(true));
        I2C0WithClock { reg: self }
    }
}

// Given that I2C0 and I2C1 registers share no commonalities in the SVD, those need to be carried
// around for each I2C. On the other hand, as long as we don't allow later extraction of pins and
// clocks, theose structs can stay as small as they are.
pub struct I2C0WithClock {
    reg: registers::I2C0,
}

impl I2C0WithClock {
    pub fn with_scl(self, route: registers::i2c0::routeloc0::SCLLOCW, scl: pins::PC11<Disabled>) -> Result<I2C0WithScl<pins::PC11<Disabled>>, ()>
    {
        // This currently fails to compile due to missing ParialEq on the write values
//         if route != registers::i2c0::routeloc0::SCLLOCW::LOC15 {
//             return Result::Err(());
//         }

        Ok(I2C0WithScl { reg: self.reg, sclroute: route, sclpin: scl })
    }
// PF0 is SCL#23 etc -- we might need to generate those for all.
}

pub struct I2C0WithScl<SCLP: EFM32Pin> {
    reg: registers::I2C0,
    // Carried around briefly in the struct so they can be enabled in the right sequence in the end
    sclpin: SCLP,
    sclroute: registers::i2c0::routeloc0::SCLLOCW,
}

impl<SCLP: EFM32Pin> I2C0WithScl<SCLP> {
    pub fn with_sda(self, route: registers::i2c0::routeloc0::SDALOCW, sda: pins::PC10<Disabled>) -> Result<ConfiguredI2C0, ()>
    {
        // This currently fails to compile due to missing ParialEq on the write values
//         if route != registers::i2c0::routeloc0::SDALOCW::LOC15 {
//             return Result::Err(());
//         }

        let sclroute = self.sclroute;
        self.reg.routeloc0.write(|w| w.sdaloc().variant(route).sclloc().variant(sclroute));
        self.reg.routepen.write(|w| w.sdapen().bit(true).sclpen().bit(true));

        let _swallowed = self.sclpin.as_opendrain();
        let _swallowed = sda.as_opendrain();

        self.reg.cmd.write(|w| w.abort().bit(true));

        Ok(ConfiguredI2C0 { reg: self.reg })
    }
    // Same would need duplication
}

pub struct ConfiguredI2C0 {
    reg: registers::I2C0,
}

#[derive(Debug)]
pub enum Error {
    // As long as no ack/nack checks are done, we don't really return any
}

impl embedded_hal::blocking::i2c::Write for ConfiguredI2C0 {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error>
    {
        self.reg.txdata.write(|w| unsafe { w.txdata().bits(addr) });
        self.reg.cmd.write(|w| w.start().bit(true));
        for datum in bytes.iter() {
            while !self.reg.status.read().txbl().bit() {};
    //         if self.reg.state.read().nacked().bit() { writeln!(stdout, "nacked at 1"); };
            self.reg.txdata.write(|w| unsafe { w.txdata().bits(*datum) });
        }
        while !self.reg.status.read().txbl().bit() {};
        while !self.reg.state.read().state().is_dataack() {}
        self.reg.cmd.write(|w| w.stop().bit(true));

        Ok(())
    }
}
