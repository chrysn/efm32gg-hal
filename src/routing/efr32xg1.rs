use crate::timer::{TimerChannel, Timer0, Channel0, Channel1, Channel2, RoutedTimerChannel};
use crate::gpio;

impl TimerChannel<Timer0, Channel0> {
    pub fn route(self, pin: gpio::pins::PD11<gpio::Output>) -> RoutedTimerChannel<Timer0, Channel0, gpio::pins::PD11<gpio::Output>> {
        // FIXME see enable method
        unsafe { &mut *self.register() }.routeloc0.modify(|_, w| w.cc0loc().loc19());
        RoutedTimerChannel {
            channel: self,
            pin
        }
    }
}

impl TimerChannel<Timer0, Channel1> {
    pub fn route(self, pin: gpio::pins::PD12<gpio::Output>) -> RoutedTimerChannel<Timer0, Channel1, gpio::pins::PD12<gpio::Output>> {
        // FIXME see enable method
        unsafe { &mut *self.register() }.routeloc0.modify(|_, w| w.cc1loc().loc19());
        RoutedTimerChannel {
            channel: self,
            pin
        }
    }
}

impl TimerChannel<Timer0, Channel2> {
    pub fn route(self, pin: gpio::pins::PD13<gpio::Output>) -> RoutedTimerChannel<Timer0, Channel2, gpio::pins::PD13<gpio::Output>> {
        // FIXME see enable method
        unsafe { &mut *self.register() }.routeloc0.modify(|_, w| w.cc2loc().loc19());
        RoutedTimerChannel {
            channel: self,
            pin
        }
    }
}
