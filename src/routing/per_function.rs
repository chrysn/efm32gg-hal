#[macro_export]
macro_rules! timerchannel_pin {
    ($TimerN: ident, $ChannelX: ident, $Pin: ident, $locI: ident, $ccXloc: ident, $ccXpen: ident) => {
        impl super::HasLocForFunction<$TimerN, $ChannelX>
            for crate::gpio::pins::$Pin<crate::gpio::Output>
        {
            unsafe fn configure() {
                // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
                let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();
                cortex_m::interrupt::free(|_| {
                    reg.routeloc0.modify(|_, w| w.$ccXloc().$locI());
                    reg.routepen.modify(|_, w| w.$ccXpen().set_bit());
                });
            }

            unsafe fn deconfigure() {
                // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
                let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();
                cortex_m::interrupt::free(|_| {
                    reg.routepen.modify(|_, w| w.$ccXpen().clear_bit());
                });
            }
        }
    };
}

pub(crate) use timerchannel_pin;
