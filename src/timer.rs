//! TIMER (Timer/Counter peripheral)
//!
//! This module exposes some features of the EFM32 timer/counter peripheral; most notably, it
//! allows easy configuration of PWM pins.

use registers;
use cmu;

pub trait TimerExt<Clk, Timer> {
    fn with_clock(self, clock: Clk) -> Timer;
}


macro_rules! timer {
    ($TIMERn: ident, $TIMERnClk: ident, $TimerN: ident, $timerN: ident) => {

impl TimerExt<cmu::$TIMERnClk, $TimerN> for registers::$TIMERn {
    fn with_clock(self, mut clock: cmu::$TIMERnClk) -> $TimerN {
        clock.enable();
        $TimerN { register: self, clock }
    }
}

pub struct $TimerN {
    register: registers::$TIMERn,
    clock: cmu::$TIMERnClk,
}

impl embedded_hal::Pwm for $TimerN {
    type Channel = i32; // FIXME needs atype
    type Time = (); // FIXME
    type Duty = u16; // FIXME check the extreme behaviors

    fn enable(&mut self, channel: Self::Channel) {
        match channel {
            0 => {
                self.register.route.modify(|_, w| w.cc0pen().set_bit());
                self.register.cc0_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc0_ctrl::MODEW::PWM));
            },
            1 => {
                self.register.route.modify(|_, w| w.cc1pen().set_bit());
                self.register.cc1_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc1_ctrl::MODEW::PWM));
            },
            2 => {
                self.register.route.modify(|_, w| w.cc2pen().set_bit());
                self.register.cc2_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc2_ctrl::MODEW::PWM));
            },
            _ => panic!("Nonexistent channel"),
        }
    }

    fn disable(&mut self, channel: Self::Channel) {
        match channel {
            0 => self.register.route.modify(|_, w| w.cc0pen().clear_bit()),
            1 => self.register.route.modify(|_, w| w.cc1pen().clear_bit()),
            2 => self.register.route.modify(|_, w| w.cc2pen().clear_bit()),
            _ => panic!("Nonexistent channel"),
        }
    }

    fn get_period(&self) -> () {
        unimplemented!();
    }

    fn set_period<P>(&mut self, time: P) {
        unimplemented!();
    }

    fn get_max_duty(&self) -> Self::Duty {
        unimplemented!();
    }

    fn set_duty(&mut self, channel: i32, duty: u16) {
        match channel {
            0 => self.register.cc0_ccv.modify(|_, w| unsafe { w.ccv().bits(duty) }),
            1 => self.register.cc1_ccv.modify(|_, w| unsafe { w.ccv().bits(duty) }),
            2 => self.register.cc2_ccv.modify(|_, w| unsafe { w.ccv().bits(duty) }),
            _ => panic!("Nonexistent channel"),
        }
    }

    fn get_duty(&self, channel: i32) -> u16 {
        unimplemented!();
    }
}
    }
}

timer!(TIMER0, TIMER0Clk, Timer0, timer0);
timer!(TIMER1, TIMER1Clk, Timer1, timer1);
timer!(TIMER2, TIMER2Clk, Timer2, timer2);

impl Timer2 {
    // FIXME This should be runnable only once and return Channel types, consuming pins
    pub fn configure_route0(&mut self) {
        self.register.route.modify(|_, w| w.location().variant(registers::timer2::route::LOCATIONW::LOC0));

        // FIXME that's not the sequence I'd usually execute, I'd rather to this later.
        self.register.cmd.write(|w| w.start().bit(true));
    }
}

impl Timer1 {
    // FIXME as above
    pub fn configure_route2(&mut self) {
        self.register.route.modify(|_, w| w.location().variant(registers::timer1::route::LOCATIONW::LOC2));

        // FIXME as above
        self.register.cmd.write(|w| w.start().bit(true));
    }
}
