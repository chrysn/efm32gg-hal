/// Route information taken from [Mighty Gecko data
/// sheet](https://www.silabs.com/documents/public/data-sheets/efr32mg1-datasheet.pdf); Flex- and
/// Blue Gecko appear to be the same.

use crate::timer::{TimerChannel, Timer0, Channel0, Channel1, Channel2, RoutedTimerChannel};
use crate::gpio;

/// Indicates that a given pin is a valid routing location for a given peripheral and function
pub trait HasLocForFunction<P, F> {
    unsafe fn configure();
}

macro_rules! timerchannel_pin {
    ($TimerN: ident, $ChannelX: ident, $ccXloc: ident, $Pin: ident, $locI: ident) => {

impl HasLocForFunction<$TimerN, $ChannelX> for gpio::pins::$Pin<gpio::Output> {
    unsafe fn configure() {
        // FIXME see enable method
        unsafe { &mut *TimerChannel::<$TimerN, $ChannelX>::register() }.routeloc0.modify(|_, w| w.$ccXloc().$locI());
    }
}

    }
}

timerchannel_pin!(Timer0, Channel0, cc0loc, PA0, loc0);

timerchannel_pin!(Timer0, Channel0, cc0loc, PD11, loc19);
timerchannel_pin!(Timer0, Channel1, cc1loc, PD12, loc19);
timerchannel_pin!(Timer0, Channel2, cc2loc, PD13, loc19);

impl<T, C> TimerChannel<T, C> {
    pub fn route<P>(self, pin: P) -> RoutedTimerChannel<T, C, P> where
        P: HasLocForFunction<T, C>,
    {
        unsafe { P::configure() };
        RoutedTimerChannel {
            channel: self,
            pin
        }
    }
}
