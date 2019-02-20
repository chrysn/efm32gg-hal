#[macro_export]
macro_rules! gpio {
    ([$(($pX_drive:ident, $pX_ctrl:ident),)+],
     [$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_dout:ident, $modei:ident, $px_modehl:ident, $outclr:ident, $outset:ident),)+]) => {

        pub mod pins {
            use embedded_hal::digital;
            use core::marker::PhantomData;
            use super::*;

            $(
                pub struct $PXi<MODE> {
                    pub(super) _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_disabled(self) -> $PXi<Disabled<Floating>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().disabled());
                        unsafe { gpio.$outclr.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_disabled_pulled_up(self) -> $PXi<Disabled<PullUp>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().disabled());
                        unsafe { gpio.$outset.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input(self) -> $PXi<Input<Floating>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().input());
                        unsafe { gpio.$outclr.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_with_filter(self) -> $PXi<Input<WithFilter<Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().input());
                        unsafe { gpio.$outset.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_down(self) -> $PXi<Input<PullDown>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().inputpull());
                        unsafe { gpio.$outclr.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_up(self) -> $PXi<Input<PullUp>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().inputpull());
                        unsafe { gpio.$outset.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_down_with_filter(self) -> $PXi<Input<WithFilter<PullDown>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().inputpullfilter());
                        unsafe { gpio.$outclr.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_up_with_filter(self) -> $PXi<Input<WithFilter<PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().inputpullfilter());
                        unsafe { gpio.$outset.write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_pushpull(self) -> $PXi<Output<PushPull<Normal>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().pushpull());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_pushpull_alt_drive(self) -> $PXi<Output<PushPull<Alternate>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().pushpulldrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredor(self) -> $PXi<Output<WiredOr<Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredor());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredor_pulled_down(self) -> $PXi<Output<WiredOr<PullDown>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredorpulldown());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand(self) -> $PXi<Output<WiredAnd<Normal, Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredand());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_pulled_up(self) -> $PXi<Output<WiredAnd<Normal, PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandpullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_with_filter(self) -> $PXi<Output<WithFilter<WiredAnd<Normal, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_with_filter_pulled_up(self) -> $PXi<Output<WithFilter<WiredAnd<Normal, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandpullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive(self) -> $PXi<Output<WiredAnd<Alternate, Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_with_filter(self) -> $PXi<Output<WithFilter<WiredAnd<Alternate, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivefilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_pulled_up(self) -> $PXi<Output<WiredAnd<Alternate, PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivepullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_with_filter_pulled_up(self) -> $PXi<Output<WithFilter<WiredAnd<Alternate, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivepullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_pushpull(self) -> $PXi<Input<Output<PushPull<Normal>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().pushpull());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_pushpull_alt_drive(self) -> $PXi<Input<Output<PushPull<Alternate>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().pushpulldrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredor(self) -> $PXi<Input<Output<WiredOr<Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredor());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredor_pulled_down(self) -> $PXi<Input<Output<WiredOr<PullDown>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredorpulldown());

                        $PXi { _mode: PhantomData }
                    }


                    pub fn into_io_wiredand(self) -> $PXi<Input<Output<WiredAnd<Normal, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredand());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_pulled_up(self) -> $PXi<Input<Output<WiredAnd<Normal, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandpullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_with_filter(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Normal, Floating>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_with_filter_pulled_up(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Normal, PullUp>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredandpullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive(self) -> $PXi<Input<Output<WiredAnd<Alternate, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_with_filter(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Alternate, Floating>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivefilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_pulled_up(self) -> $PXi<Input<Output<WiredAnd<Alternate, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivepullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_with_filter_pulled_up(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Alternate, PullUp>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl.modify(|_, w| w.$modei().wiredanddrivepullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                }

                impl<P> digital::OutputPin for $PXi<Output<P>> {
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
                impl<P> digital::StatefulOutputPin for $PXi<Output<P>> {
                    fn is_set_low(self: &Self) -> bool {
                        let gpio = sneak_into_gpio();
                        gpio.$px_dout.read().bits() & (1 << $i) == 0
                    }

                    fn is_set_high(self: &Self) -> bool {
                        !self.is_set_low()
                    }
                }

                #[cfg(feature = "unproven")]
                impl<P> digital::InputPin for $PXi<Input<P>> {
                    fn is_low(self: &Self) -> bool {
                        let gpio = sneak_into_gpio();
                        gpio.$px_din.read().bits() & (1 << $i) == 0
                    }

                    fn is_high(self: &Self) -> bool {
                        !self.is_low()
                    }
                }

                impl<P> digital::OutputPin for $PXi<Input<Output<P>>> {
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
                impl<P> digital::StatefulOutputPin for $PXi<Input<Output<P>>> {
                    fn is_set_low(self: &Self) -> bool {
                        let gpio = sneak_into_gpio();
                        gpio.$px_dout.read().bits() & (1 << $i) == 0
                    }

                    fn is_set_high(self: &Self) -> bool {
                        !self.is_set_low()
                    }
                }

                )+
        }

        /// Ports, unlike Pins, doesn't need to be abstractized as a set of bank/port
        /// we're only using this for extra configuration at bank/port level.
        /// Ports is consumed after another split call.
        pub struct Ports;

        pub struct Pins {
            $(
                pub $pxi: pins::$PXi<Disabled<Floating>>,
                )+
        }

        impl Ports {
            /// when Ports splitted, give Pins back.
            /// Ports then will be consumed implicitly.
            pub fn split(self) -> Pins {
                Pins {
                    $(
                        $pxi: pins::$PXi { _mode: PhantomData },
                        )+
                }
            }

            $(
                pub fn $pX_drive(mut self, mode: DriveMode) -> Self {
                    let gpio = sneak_into_gpio();
                    gpio.$pX_ctrl.write(|w| w.drivemode().bits(mode._bits()));
                    self
                }
            )+
        }

        /// Parts has both pins and ports, but designed so only one or the ther
        /// can be used. Its fields are private, and one field will be not available
        /// when the other is used/moved.
        pub struct Parts {
            ports: Ports,
            pins: Pins,
        }

        impl Parts {
            /// Move Ports, consume Parts and pins implicitly.
            pub fn ports(self) -> Ports {
                self.ports
            }

            /// Move Pins, consume Parts and ports implicitly.
            pub fn pins(self) -> Pins {
                self.pins
            }
        }

        pub trait GPIOExt {
            fn split(self, gpioclk: cmu::GPIOClk) -> Parts;
        }

        impl GPIOExt for registers::GPIO {
            fn split(self, mut gpioclk: cmu::GPIOClk) -> Parts {
                // The GPIO register block gets consumed, further access only happens through the
                // pins we're giving out.
                let _consumed = self;

                gpioclk.enable();

                Parts {
                    ports: Ports,
                    pins: Pins {
                        $(
                            $pxi: pins::$PXi { _mode: PhantomData },
                            )+
                    }
                }
            }
        }
    }
}
