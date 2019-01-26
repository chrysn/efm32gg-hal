use crate::timer::{TimerChannel, Timer1, Timer2};
use crate::gpio;

impl Timer2 {
    // FIXME This should work more on type level, return Channel types and consume pins
    pub fn configure_route0(
        &mut self,
//         cc0: Option<>,
//         cc1: Option<>,
//         cc2: Option<>,
    ) {
        self.register.route.modify(|_, w| w.location().variant(registers::timer2::route::LOCATIONW::LOC0));

        // FIXME that's not the sequence I'd usually execute, I'd rather to this later.
        self.register.cmd.write(|w| w.start().bit(true));
    }
}

impl Timer1 {
    // FIXME as above
    pub fn configure_route2(
            &mut self,
            _cc0: gpio::pins::PB0<gpio::Output>,
            _cc1: gpio::pins::PB1<gpio::Output>,
            _cc2: gpio::pins::PB2<gpio::Output>
    ) {
        self.register.route.modify(|_, w| w.location().variant(registers::timer1::route::LOCATIONW::LOC2));

        // FIXME as above
        self.register.cmd.write(|w| w.start().bit(true));
    }
}
