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

// The full list of the EFM32GG990 data sheet, which should be the most comprehensive version. (The
// EFM32GG series was not observed to have any conflicts between its members, so if anything is
// missing here that's present on another GG, it should be possible to just add it.)

use crate::timer::{Timer0, Timer3, Channel0, Channel1, Channel2};
use super::per_peripheral::timerperipheral_pin;

timerperipheral_pin!(Timer0, Channel0, PA0, is_loc0, cc0_ctrl);
// timerperipheral_pin!(Timer0, Channel0, PA0, is_loc1, cc0_ctrl);
timerperipheral_pin!(Timer0, Channel0, PF6, is_loc2, cc0_ctrl);
timerperipheral_pin!(Timer0, Channel0, PD1, is_loc3, cc0_ctrl);
// timerperipheral_pin!(Timer0, Channel0, PA0, is_loc4, cc0_ctrl);
timerperipheral_pin!(Timer0, Channel0, PF0, is_loc5, cc0_ctrl);

timerperipheral_pin!(Timer0, Channel1, PA1, is_loc0, cc1_ctrl);
// timerperipheral_pin!(Timer0, Channel1, PA1, is_loc1, cc1_ctrl);
timerperipheral_pin!(Timer0, Channel1, PF7, is_loc2, cc1_ctrl);
timerperipheral_pin!(Timer0, Channel1, PD2, is_loc3, cc1_ctrl);
timerperipheral_pin!(Timer0, Channel1, PC0, is_loc4, cc1_ctrl);
timerperipheral_pin!(Timer0, Channel1, PF1, is_loc5, cc1_ctrl);

timerperipheral_pin!(Timer0, Channel2, PA2, is_loc0, cc2_ctrl);
// timerperipheral_pin!(Timer0, Channel2, PA2, is_loc1, cc2_ctrl);
timerperipheral_pin!(Timer0, Channel2, PF8, is_loc2, cc2_ctrl);
timerperipheral_pin!(Timer0, Channel2, PD3, is_loc3, cc2_ctrl);
timerperipheral_pin!(Timer0, Channel2, PC1, is_loc4, cc2_ctrl);
timerperipheral_pin!(Timer0, Channel2, PF2, is_loc5, cc2_ctrl);

timerperipheral_pin!(Timer1, Channel0, PE10, is_loc1, cc0_ctrl);
timerperipheral_pin!(Timer1, Channel0, PB0, is_loc2, cc0_ctrl);
timerperipheral_pin!(Timer1, Channel0, PB7, is_loc3, cc0_ctrl);
timerperipheral_pin!(Timer1, Channel0, PD6, is_loc4, cc0_ctrl);

timerperipheral_pin!(Timer1, Channel1, PE11, is_loc1, cc1_ctrl);
timerperipheral_pin!(Timer1, Channel1, PB1, is_loc2, cc1_ctrl);
timerperipheral_pin!(Timer1, Channel1, PB8, is_loc3, cc1_ctrl);
timerperipheral_pin!(Timer1, Channel1, PD7, is_loc4, cc1_ctrl);

timerperipheral_pin!(Timer1, Channel2, PE12, is_loc1, cc2_ctrl);
timerperipheral_pin!(Timer1, Channel2, PB2, is_loc2, cc2_ctrl);
timerperipheral_pin!(Timer1, Channel2, PB11, is_loc3, cc2_ctrl);

timerperipheral_pin!(Timer2, Channel0, PA8, is_loc0, cc0_ctrl);
timerperipheral_pin!(Timer2, Channel0, PA12, is_loc1, cc0_ctrl);
timerperipheral_pin!(Timer2, Channel0, PC8, is_loc2, cc0_ctrl);

timerperipheral_pin!(Timer2, Channel1, PA9, is_loc0, cc1_ctrl);
timerperipheral_pin!(Timer2, Channel1, PA13, is_loc1, cc1_ctrl);
timerperipheral_pin!(Timer2, Channel1, PC9, is_loc2, cc1_ctrl);

timerperipheral_pin!(Timer2, Channel2, PA10, is_loc0, cc2_ctrl);
timerperipheral_pin!(Timer2, Channel2, PA14, is_loc1, cc2_ctrl);
timerperipheral_pin!(Timer2, Channel2, PC10, is_loc2, cc2_ctrl);

timerperipheral_pin!(Timer3, Channel0, PE14, is_loc0, cc0_ctrl);
timerperipheral_pin!(Timer3, Channel0, PE0, is_loc1, cc0_ctrl);

timerperipheral_pin!(Timer3, Channel1, PE15, is_loc0, cc1_ctrl);
timerperipheral_pin!(Timer3, Channel1, PE1, is_loc1, cc1_ctrl);

timerperipheral_pin!(Timer3, Channel2, PA15, is_loc0, cc2_ctrl);
timerperipheral_pin!(Timer3, Channel2, PE2, is_loc1, cc2_ctrl);
