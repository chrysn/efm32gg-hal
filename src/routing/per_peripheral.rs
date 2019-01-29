#[macro_export]
macro_rules! timerperipheral_pin {
    ($TimerN: ident, $ChannelX: ident, $Pin: ident, $is_locI: ident, $ccX_ctrl: ident) => {

impl super::HasLocForFunction<$TimerN, $ChannelX> for crate::gpio::pins::$Pin<crate::gpio::Output> {
    unsafe fn configure() {
        // unsafe: See the individual accesses on reg
        let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();

        // This is safe because it's read-only access
        assert!(reg.route.read().location().$is_locI(), "Pin was not adaequately pre-routed");

        // This is a safe access because it only acts on a ccX register
        reg.$ccX_ctrl.modify(|_, w| w.mode().pwm());
    }
}

    }
}

pub(crate) use timerperipheral_pin;
