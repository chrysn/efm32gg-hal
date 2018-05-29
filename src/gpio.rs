use efm32gg990;
use embedded_hal::digital;

use core::marker::PhantomData;

pub struct Disabled {}
pub struct Output {}
pub struct Input {}

pub struct PA0<Mode> {
    _mode: PhantomData<Mode>,
}

impl digital::OutputPin for PA0<Output> {
    fn set_low(self: &mut Self) {
        let gpio = unsafe { efm32gg990::Peripherals::steal() }.GPIO;
        gpio.pa_doutclr.write(|w| unsafe { w.bits(1 << 0) });
    }

    fn set_high(self: &mut Self) {
        let gpio = unsafe { efm32gg990::Peripherals::steal() }.GPIO;
        gpio.pa_doutset.write(|w| unsafe { w.bits(1 << 0) });
    }
}
#[cfg(feature = "unproven")]
impl digital::InputPin for PA0<Input> {
    fn is_low(self: &Self) -> bool {
        let gpio = unsafe { efm32gg990::Peripherals::steal() }.GPIO;
        gpio.pa_din.read().bits() & (1 << 0) == 0
    }

    fn is_high(self: &Self) -> bool {
        !self.is_low()
    }
}

impl<Mode> PA0<Mode> {
    pub fn as_output(self: Self) -> PA0<Output> {
        let gpio = unsafe { efm32gg990::Peripherals::steal() }.GPIO;
        gpio.pa_model.modify(|_, w| w.mode0().pushpull());

        PA0 { _mode: PhantomData }
    }
}

impl<Mode> PA0<Mode> {
    pub fn as_input(self: Self) -> PA0<Input> {
        let gpio = unsafe { efm32gg990::Peripherals::steal() }.GPIO;
        gpio.pa_model.modify(|_, w| w.mode0().input());

        PA0 { _mode: PhantomData }
    }
}

pub struct Pins {
    pub pa0: PA0<Disabled>,
}

pub fn split(_comsumed_peripheral: efm32gg990::GPIO) -> Pins {
    Pins {
        pa0: PA0 { _mode: PhantomData },
    }
}



// impl<Port> digital::OutputPin for SpecificOutputPin<port=Port> {
//     fn set_low(self: &mut Self) {
//         let doutclr = Self::doutclr;
//     }
// }
// 
// 
// struct PortA { }
// 
// impl Port for PortA {
//     type din = efm32gg990::gpio::PA_DIN;
//     type doutset = efm32gg990::gpio::PA_DOUTSET;
//     type doutclr = efm32gg990::gpio::PA_DOUTCLR;
//     type model = efm32gg990::gpio::PA_MODEL;
//     type modeh = efm32gg990::gpio::PA_MODEH;
// }
// 
// struct PA0 {}
// 
// impl<Mode> SpecificPin<Mode> for PA0 {
//     const NUMBER: u8 = 0;
//     type port = PortA;
// }

// // pin-number fixed pin
// pub struct PA0<MODE> {
//     _mode: PhantomData<MODE>,
// }
// 
// 
// pub trait GpioExt {
//     fn split(self, cmu: &mut CMU) -> Self::Parts;
// }
// 
// // stm32 splits this into modules; trying how far i get without.
// pub struct PAParts {
//     pub pa0: PA0,
// }


