//! GPIO (general purpose input/output), mapped to embedded_hal::digital
//!
//! This implements only what is minimally essential to make input or output pins out of the GPIO
//! register block. Other drive modes could be added with relative ease (eg. Wired-And, Wired-Or,
//! input with pull-up/-down), others (eg. the per-bank drive strength, EM4 wakeup) will need
//! additional mechanisms, and some (eg. clearing the configuration lock) might need changes to the
//! whole model if at all desired.

use registers;

use core::marker::PhantomData;
use embedded_hal::digital;

use bitband;

pub struct Disabled {}
pub struct Output {}
pub struct Input {}

pub trait GPIOExt {
    fn split(self, cmu: &mut registers::CMU) -> Pins;
}

/// A trait pertinent to a single GPIO pin; this trait exposes all the functionality that is not
/// exposed via the embedded_hal::digital traits.
///
/// Given that different modes of operation make sense with different APIs (a pin configured for
/// output can be driven, one for input can be queried), the trait is not implemented by a single
/// struct for a pin but qualified by a parameterized struct whose expressions implement different
/// traits; for example, PA0<T> implements EFM32Pin for all T, and configurin it as an output pin
/// with `.as_output()` makes it into a PA0<Output> which implements digital::OutputPin.
///
/// Currently, not all information about a pin is encoded in its type (For example, as_output and
/// as_opendrain produce the same type), that may change just by adding an InputOutput
/// configuration state that has implements both input and output traits (as it makes sense to both
/// write to and read an open drain pin), or may even go as far as to encode the full pin
/// configuration.
pub trait EFM32Pin {

    type Disabled;
    type Output: digital::OutputPin;
    type Input: digital::InputPin;

    /// Convert the pin into an output pin. The original pin, however configured, is consumed, the
    /// hardware configuration changed to drive high or low, and returned as a pin that implements
    /// embedded_hal::digital::OutputPin.
    fn as_output(self: Self) -> Self::Output;

    /// Convert the pin into an open drain (wired "and") pin. The original pin, however configured,
    /// is consumed, the hardware configuration changed to only drive low, and returned as a pin
    /// that implements embedded_hal::digital::OutputPin (and should later implement InputPin too,
    /// buit that needs some thinking-through anyway w/rt how much of the configure state should be
    /// in the typ).
    fn as_opendrain(self: Self) -> Self::Output;

    /// Convert the pin into an input pin. The original pin, however configured, is consumed, the
    /// hardware configuration changed to input with no pull-up- or down resistors, and returned as
    /// a pin that implements embedded_hal::digital::InputPin.
    fn as_input(self: Self) -> Self::Input;
}

fn sneak_into_gpio() -> &'static registers::gpio::RegisterBlock {
        unsafe { &*registers::GPIO::ptr() }
}

macro_rules! gpio {
    ([$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_dout:ident, $modei:ident, $px_modehl:ident),)+]) => {

        pub mod pins {
            use embedded_hal::digital;
            use core::marker::PhantomData;
            use super::*;

            $(
                pub struct $PXi<Mode> {
                    pub(super) _mode: PhantomData<Mode>,
                }

                impl digital::OutputPin for $PXi<Output> {
                    fn set_low(self: &mut Self) {
                        // This implementation uses bit-banding on all EFx32 devices. EFM2 would
                        // have explicit set/clear registers, but bit-banding is available there
                        // too and I don't expect any performance difference.

                        let gpio = sneak_into_gpio();
                        // unsafe: We "own" that pin and thus that bit in the register, and
                        // bit-band writing is atomic even though others might access the register
                        // simultaneously.
                        unsafe { bitband::change_bit(&gpio.$px_dout, $i, false); }
                    }

                    fn set_high(self: &mut Self) {
                        // see comments on set_low
                        let gpio = sneak_into_gpio();
                        unsafe { bitband::change_bit(&gpio.$px_dout, $i, true); }
                    }
                }
                #[cfg(feature = "unproven")]
                impl digital::InputPin for $PXi<Input> {
                    fn is_low(self: &Self) -> bool {
                        let gpio = sneak_into_gpio();
                        gpio.$px_din.read().bits() & (1 << $i) == 0
                    }

                    fn is_high(self: &Self) -> bool {
                        !self.is_low()
                    }
                }

                impl<Mode> EFM32Pin for $PXi<Mode> {
                    type Disabled = $PXi<Disabled>;
                    type Output = $PXi<Output>;
                    type Input = $PXi<Input>;

                    fn as_output(self: Self) -> Self::Output {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().pushpull());

                        $PXi { _mode: PhantomData }
                    }
                    fn as_opendrain(self: Self) -> Self::Output {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredand());

                        $PXi { _mode: PhantomData }
                    }
                    fn as_input(self: Self) -> Self::Input {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().input());

                        $PXi { _mode: PhantomData }
                    }
                }
            )+
        }

        pub struct Pins {
            $(
                pub $pxi: pins::$PXi<Disabled>,
            )+
        }

        impl GPIOExt for registers::GPIO {
            fn split(self, cmu: &mut registers::CMU) -> Pins {
                // The GPIO register block gets consumed, further access only happens through the
                // pins we're giving out.
                let _consumed = self;

                // A later version will likely want to use a CMU abstraction.
                #[cfg(feature = "chip-efm32gg")]
                cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));
                #[cfg(feature = "chip-efr32xg1")]
                cmu.hfbusclken0.modify(|_, w| w.gpio().bit(true));

                Pins {
                    $(
                        $pxi: pins::$PXi { _mode: PhantomData },
                    )+
                }
            }
        }
    }
}



gpio!([
    PA0:  (pa0,  0,  pa_din, pa_dout, mode0,  pa_model),
    PA1:  (pa1,  1,  pa_din, pa_dout, mode1,  pa_model),
    PA2:  (pa2,  2,  pa_din, pa_dout, mode2,  pa_model),
    PA3:  (pa3,  3,  pa_din, pa_dout, mode3,  pa_model),
    PA4:  (pa4,  4,  pa_din, pa_dout, mode4,  pa_model),
    PA5:  (pa5,  5,  pa_din, pa_dout, mode5,  pa_model),
    PA6:  (pa6,  6,  pa_din, pa_dout, mode6,  pa_model),
    PA7:  (pa7,  7,  pa_din, pa_dout, mode7,  pa_model),
    PA8:  (pa8,  8,  pa_din, pa_dout, mode8,  pa_modeh),
    PA9:  (pa9,  9,  pa_din, pa_dout, mode9,  pa_modeh),
    PA10: (pa10, 10, pa_din, pa_dout, mode10, pa_modeh),
    PA11: (pa11, 11, pa_din, pa_dout, mode11, pa_modeh),
    PA12: (pa12, 12, pa_din, pa_dout, mode12, pa_modeh),
    PA13: (pa13, 13, pa_din, pa_dout, mode13, pa_modeh),
    PA14: (pa14, 14, pa_din, pa_dout, mode14, pa_modeh),
    PA15: (pa15, 15, pa_din, pa_dout, mode15, pa_modeh),
    PB0:  (pb0,  0,  pb_din, pb_dout, mode0,  pb_model),
    PB1:  (pb1,  1,  pb_din, pb_dout, mode1,  pb_model),
    PB2:  (pb2,  2,  pb_din, pb_dout, mode2,  pb_model),
    PB3:  (pb3,  3,  pb_din, pb_dout, mode3,  pb_model),
    PB4:  (pb4,  4,  pb_din, pb_dout, mode4,  pb_model),
    PB5:  (pb5,  5,  pb_din, pb_dout, mode5,  pb_model),
    PB6:  (pb6,  6,  pb_din, pb_dout, mode6,  pb_model),
    PB7:  (pb7,  7,  pb_din, pb_dout, mode7,  pb_model),
    PB8:  (pb8,  8,  pb_din, pb_dout, mode8,  pb_modeh),
    PB9:  (pb9,  9,  pb_din, pb_dout, mode9,  pb_modeh),
    PB10: (pb10, 10, pb_din, pb_dout, mode10, pb_modeh),
    PB11: (pb11, 11, pb_din, pb_dout, mode11, pb_modeh),
    PB12: (pb12, 12, pb_din, pb_dout, mode12, pb_modeh),
    PB13: (pb13, 13, pb_din, pb_dout, mode13, pb_modeh),
    PB14: (pb14, 14, pb_din, pb_dout, mode14, pb_modeh),
    PB15: (pb15, 15, pb_din, pb_dout, mode15, pb_modeh),
    PC0:  (pc0,  0,  pc_din, pc_dout, mode0,  pc_model),
    PC1:  (pc1,  1,  pc_din, pc_dout, mode1,  pc_model),
    PC2:  (pc2,  2,  pc_din, pc_dout, mode2,  pc_model),
    PC3:  (pc3,  3,  pc_din, pc_dout, mode3,  pc_model),
    PC4:  (pc4,  4,  pc_din, pc_dout, mode4,  pc_model),
    PC5:  (pc5,  5,  pc_din, pc_dout, mode5,  pc_model),
    PC6:  (pc6,  6,  pc_din, pc_dout, mode6,  pc_model),
    PC7:  (pc7,  7,  pc_din, pc_dout, mode7,  pc_model),
    PC8:  (pc8,  8,  pc_din, pc_dout, mode8,  pc_modeh),
    PC9:  (pc9,  9,  pc_din, pc_dout, mode9,  pc_modeh),
    PC10: (pc10, 10, pc_din, pc_dout, mode10, pc_modeh),
    PC11: (pc11, 11, pc_din, pc_dout, mode11, pc_modeh),
    PC12: (pc12, 12, pc_din, pc_dout, mode12, pc_modeh),
    PC13: (pc13, 13, pc_din, pc_dout, mode13, pc_modeh),
    PC14: (pc14, 14, pc_din, pc_dout, mode14, pc_modeh),
    PC15: (pc15, 15, pc_din, pc_dout, mode15, pc_modeh),
    PD0:  (pd0,  0,  pd_din, pd_dout, mode0,  pd_model),
    PD1:  (pd1,  1,  pd_din, pd_dout, mode1,  pd_model),
    PD2:  (pd2,  2,  pd_din, pd_dout, mode2,  pd_model),
    PD3:  (pd3,  3,  pd_din, pd_dout, mode3,  pd_model),
    PD4:  (pd4,  4,  pd_din, pd_dout, mode4,  pd_model),
    PD5:  (pd5,  5,  pd_din, pd_dout, mode5,  pd_model),
    PD6:  (pd6,  6,  pd_din, pd_dout, mode6,  pd_model),
    PD7:  (pd7,  7,  pd_din, pd_dout, mode7,  pd_model),
    PD8:  (pd8,  8,  pd_din, pd_dout, mode8,  pd_modeh),
    PD9:  (pd9,  9,  pd_din, pd_dout, mode9,  pd_modeh),
    PD10: (pd10, 10, pd_din, pd_dout, mode10, pd_modeh),
    PD11: (pd11, 11, pd_din, pd_dout, mode11, pd_modeh),
    PD12: (pd12, 12, pd_din, pd_dout, mode12, pd_modeh),
    PD13: (pd13, 13, pd_din, pd_dout, mode13, pd_modeh),
    PD14: (pd14, 14, pd_din, pd_dout, mode14, pd_modeh),
    PD15: (pd15, 15, pd_din, pd_dout, mode15, pd_modeh),
    PE0:  (pe0,  0,  pe_din, pe_dout, mode0,  pe_model),
    PE1:  (pe1,  1,  pe_din, pe_dout, mode1,  pe_model),
    PE2:  (pe2,  2,  pe_din, pe_dout, mode2,  pe_model),
    PE3:  (pe3,  3,  pe_din, pe_dout, mode3,  pe_model),
    PE4:  (pe4,  4,  pe_din, pe_dout, mode4,  pe_model),
    PE5:  (pe5,  5,  pe_din, pe_dout, mode5,  pe_model),
    PE6:  (pe6,  6,  pe_din, pe_dout, mode6,  pe_model),
    PE7:  (pe7,  7,  pe_din, pe_dout, mode7,  pe_model),
    PE8:  (pe8,  8,  pe_din, pe_dout, mode8,  pe_modeh),
    PE9:  (pe9,  9,  pe_din, pe_dout, mode9,  pe_modeh),
    PE10: (pe10, 10, pe_din, pe_dout, mode10, pe_modeh),
    PE11: (pe11, 11, pe_din, pe_dout, mode11, pe_modeh),
    PE12: (pe12, 12, pe_din, pe_dout, mode12, pe_modeh),
    PE13: (pe13, 13, pe_din, pe_dout, mode13, pe_modeh),
    PE14: (pe14, 14, pe_din, pe_dout, mode14, pe_modeh),
    PE15: (pe15, 15, pe_din, pe_dout, mode15, pe_modeh),
    PF0:  (pf0,  0,  pf_din, pf_dout, mode0,  pf_model),
    PF1:  (pf1,  1,  pf_din, pf_dout, mode1,  pf_model),
    PF2:  (pf2,  2,  pf_din, pf_dout, mode2,  pf_model),
    PF3:  (pf3,  3,  pf_din, pf_dout, mode3,  pf_model),
    PF4:  (pf4,  4,  pf_din, pf_dout, mode4,  pf_model),
    PF5:  (pf5,  5,  pf_din, pf_dout, mode5,  pf_model),
    PF6:  (pf6,  6,  pf_din, pf_dout, mode6,  pf_model),
    PF7:  (pf7,  7,  pf_din, pf_dout, mode7,  pf_model),
    PF8:  (pf8,  8,  pf_din, pf_dout, mode8,  pf_modeh),
    PF9:  (pf9,  9,  pf_din, pf_dout, mode9,  pf_modeh),
    PF10: (pf10, 10, pf_din, pf_dout, mode10, pf_modeh),
    PF11: (pf11, 11, pf_din, pf_dout, mode11, pf_modeh),
    PF12: (pf12, 12, pf_din, pf_dout, mode12, pf_modeh),
    PF13: (pf13, 13, pf_din, pf_dout, mode13, pf_modeh),
    PF14: (pf14, 14, pf_din, pf_dout, mode14, pf_modeh),
    PF15: (pf15, 15, pf_din, pf_dout, mode15, pf_modeh),
]);
