#[macro_export]
macro_rules! timerchannel_pin {
    ($TimerN: ident, $ChannelX: ident, $Pin: ident, $locI: ident, $ccXloc: ident, $ccX_ctrl: ident) => {

impl super::HasLocForFunction<$TimerN, $ChannelX> for crate::gpio::pins::$Pin<crate::gpio::Output> {
    unsafe fn configure() {
        // unsafe: See the individual accesses on reg
        let reg = &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register();

        // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
        cortex_m::interrupt::free(|_| {
            reg.routeloc0.modify(|_, w| w.$ccXloc().$locI());
        });

        // This is a safe access because it only acts on a ccX register
        reg.$ccX_ctrl.modify(|_, w| w.mode().pwm());
    }
}

    }
}

pub use timerchannel_pin;
