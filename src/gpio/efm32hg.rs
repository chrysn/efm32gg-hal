#[macro_export]
macro_rules! gpio {
    ([$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_dout:ident, $modei:ident, $px_modehl:ident, $outclr:ident, $outset:ident),)+]) => {

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
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outclr.write(|w| w.bits(1 << $i)); }
                    }

                    fn set_high(self: &mut Self) {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outset.write(|w| w.bits(1 << $i)); }
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
