#[macro_export]
macro_rules! timerperipheral_pin {
    ($TimerN: ident, $ChannelX: ident, $Pin: ident, $is_locI: ident, $ccXpen: ident) => {

impl super::HasLocForFunction<$TimerN, $ChannelX> for crate::gpio::pins::$Pin<crate::gpio::Output> {
    unsafe fn configure() {
        // This is safe because it's read-only access
        let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();
        assert!(reg.route.read().location().$is_locI(), "Pin was not adaequately pre-routed");

        // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
        cortex_m::interrupt::free(|_| {
            reg.route.modify(|_, w| w.$ccXpen().set_bit());
        });
    }

    unsafe fn deconfigure() {
        // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
        let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();
        cortex_m::interrupt::free(|_| {
            reg.route.modify(|_, w| w.$ccXpen().clear_bit());
        });
    }
}

    }
}

pub(crate) use timerperipheral_pin;
