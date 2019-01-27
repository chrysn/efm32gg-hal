#[macro_export]
macro_rules! timerchannel_pin {
    ($TimerN: ident, $ChannelX: ident, $ccXloc: ident, $Pin: ident, $locI: ident) => {

impl super::HasLocForFunction<$TimerN, $ChannelX> for crate::gpio::pins::$Pin<crate::gpio::Output> {
    unsafe fn configure() {
        // FIXME see enable method
        unsafe { &mut *crate::timer::TimerChannel::<$TimerN, $ChannelX>::register() }.routeloc0.modify(|_, w| w.$ccXloc().$locI());
    }
}

    }
}

pub use timerchannel_pin;
