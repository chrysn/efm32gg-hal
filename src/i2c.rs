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

use super::cmu;
use gpio::{pins, Disabled, EFM32Pin};

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
    pub fn with_scl(
        self,
        route: registers::i2c0::routeloc0::SCLLOCW,
        scl: pins::PC11<Disabled>,
    ) -> Result<I2C0WithScl<pins::PC11<Disabled>>, ()> {
        if route != registers::i2c0::routeloc0::SCLLOCW::LOC15 {
            return Result::Err(());
        }

        Ok(I2C0WithScl {
            reg: self.reg,
            sclroute: route,
            sclpin: scl,
        })
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
    pub fn with_sda(
        self,
        route: registers::i2c0::routeloc0::SDALOCW,
        sda: pins::PC10<Disabled>,
    ) -> Result<ConfiguredI2C0, ()> {
        if route != registers::i2c0::routeloc0::SDALOCW::LOC15 {
            return Result::Err(());
        }

        let sclroute = self.sclroute;
        self.reg
            .routeloc0
            .write(|w| w.sdaloc().variant(route).sclloc().variant(sclroute));
        self.reg
            .routepen
            .write(|w| w.sdapen().bit(true).sclpen().bit(true));

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

/// Error conditions a read or write operation can end with. Some of those states can happen
/// regularly (eg. lost arbitration in multi-master setups), some should be prevented by this
/// implementation (eg. "Device not in idle state").
///
/// Error descriptions sometimes relate to the state diagrams of the reference manuals.
#[derive(Debug)]
pub enum Error {
    /// Device is not in idle or busy state when operation is started.
    NotReady,
    /// Arbitration was lost during transmission, another master took control of the bus.
    ArbitrationLost,
    /// The address sent was not acknowledged by any recipient.
    AddressNack,
    /// A byte sent was not acknowledged by the recipient.
    DataNack,
}

impl ConfiguredI2C0 {
    /// Set stop condition on bus and wait for bus to return to idle (or busy, if someone else
    /// starts talking just as we release) state.
    fn stop_and_finish(&mut self) {
        self.reg.cmd.write(|w| w.stop().bit(true));
        while self.reg.state.read().bits() > 1 {}
    }
}

impl embedded_hal::blocking::i2c::Write for ConfiguredI2C0 {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        //! Implemented according to diagram 17.15 of the EFR32xG1 Reference Manual rev 1.1
        //! <https://www.silabs.com/documents/public/reference-manuals/efr32xg1-rm.pdf>.  States
        //! are expressed as hex numbers for easier correlation with that documentation.
        //!
        //! It is not trying to queue up characters (thus favoring simplicity over speed), and does
        //! not allow for configured slave addresses on the master (thus avoiding to enter the
        //! slave states via the 0x73/0x71 sttes).

        if self.reg.state.read().bits() > 1 {
            return Err(Error::NotReady);
        }

        self.reg.cmd.write(|w| w.start().bit(true));

        while match self.reg.state.read().bits() {
            // The digram does not show state 0x53; it appears that if the peripheral does not know yet
            // it'd be sending, it does not set the TRANSMITTER flag
            0x53 => false,
            0x57 => false,
            _ => true,
        } {}

        self.reg
            .txdata
            .write(|w| unsafe { w.txdata().bits(addr << 1) });

        while match self.reg.state.read().bits() {
            1 => return Err(Error::ArbitrationLost),
            0x9f => {
                self.reg.cmd.write(|w| w.stop().bit(true));
                self.stop_and_finish();
                return Err(Error::AddressNack);
            }
            0x97 => false,
            _ => true,
        } {}

        for datum in bytes.iter() {
            self.reg
                .txdata
                .write(|w| unsafe { w.txdata().bits(*datum) });

            while match self.reg.state.read().bits() {
                1 => return Err(Error::ArbitrationLost),
                0xdf => {
                    self.reg.cmd.write(|w| w.stop().bit(true));
                    self.stop_and_finish();
                    return Err(Error::DataNack);
                }
                0xd7 => false,
                _ => true,
            } {}
        }

        self.stop_and_finish();

        Ok(())
    }
}

impl embedded_hal::blocking::i2c::Read for ConfiguredI2C0 {
    type Error = Error;

    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Error> {
        //! Unlike the write operation, this does not really look like the master diagram
        //! referenced there; this is partially because the workflow suggested there ("93 requires
        //! action RXDATA", even though no data has been received) is illogical, and because the
        //! diagram fails to capture that the last read must be NACK'd or even a STOP will not
        //! return the bus to idle.

        if self.reg.state.read().bits() > 1 {
            return Err(Error::NotReady);
        }

        self.reg.cmd.write(|w| w.start().bit(true));

        while match self.reg.state.read().bits() {
            // The digram does not show state 0x53; it appears that if the peripheral does not know yet
            // it'd be sending, it does not set the TRANSMITTER flag
            0x53 => false,
            0x57 => false,
            _ => true,
        } {}

        self.reg
            .txdata
            .write(|w| unsafe { w.txdata().bits((addr << 1) | 1) });

        // Happily accepting patches for a more idiomatic "ack all but the last one" expression.
        let mut i = 0;
        let imax = bytes.len();
        for mut datum in bytes.iter_mut() {
            while match self.reg.state.read().bits() {
                1 => return Err(Error::ArbitrationLost),
                0x9b => {
                    self.reg.cmd.write(|w| w.stop().bit(true));
                    self.stop_and_finish();
                    return Err(Error::AddressNack);
                }
                0xb3 => false,
                _ => true,
            } {}

            *datum = self.reg.rxdata.read().bits() as u8;

            i += 1;
            if i == imax {
                self.reg.cmd.write(|w| w.nack().bit(true));
            } else {
                self.reg.cmd.write(|w| w.ack().bit(true));
            }
        }

        self.stop_and_finish();

        Ok(())
    }
}
