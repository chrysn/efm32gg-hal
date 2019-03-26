#[macro_export]
macro_rules! gpio {
    ([$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_dout:ident, $modei:ident, $px_modehl:ident),)+]) => {

        pub mod pins {
            use embedded_hal::digital;
            use core::marker::PhantomData;
            use crate::bitband;
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
                impl digital::StatefulOutputPin for $PXi<Output> {
                    fn is_set_low(self: &Self) -> bool {
                        let gpio = sneak_into_gpio();
                        gpio.$px_dout.read().bits() & (1 << $i) == 0
                    }

                    fn is_set_high(self: &Self) -> bool {
                        !self.is_set_low()
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
            fn split(self, mut gpioclk: cmu::GPIOClk) -> Pins {
                // The GPIO register block gets consumed, further access only happens through the
                // pins we're giving out.
                let _consumed = self;

                gpioclk.enable();

                Pins {
                    $(
                        $pxi: pins::$PXi { _mode: PhantomData },
                    )+
                }
            }
        }
    }
}
