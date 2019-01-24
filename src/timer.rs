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
                #[cfg(feature = "chip-efm32gg")]
                self.register.route.modify(|_, w| w.cc0pen().set_bit());
                #[cfg(feature = "chip-efr32xg1")]
                self.register.routepen.modify(|_, w| w.cc0pen().set_bit());
                self.register.cc0_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc0_ctrl::MODEW::PWM));
            },
            1 => {
                #[cfg(feature = "chip-efm32gg")]
                self.register.route.modify(|_, w| w.cc1pen().set_bit());
                #[cfg(feature = "chip-efr32xg1")]
                self.register.routepen.modify(|_, w| w.cc1pen().set_bit());
                self.register.cc1_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc1_ctrl::MODEW::PWM));
            },
            2 => {
                #[cfg(feature = "chip-efm32gg")]
                self.register.route.modify(|_, w| w.cc2pen().set_bit());
                #[cfg(feature = "chip-efr32xg1")]
                self.register.routepen.modify(|_, w| w.cc2pen().set_bit());
                self.register.cc2_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc2_ctrl::MODEW::PWM));
            },
            _ => panic!("Nonexistent channel"),
        }
    }

    #[cfg(feature = "chip-efm32gg")]
    fn disable(&mut self, channel: Self::Channel) {
        match channel {
            0 => self.register.route.modify(|_, w| w.cc0pen().clear_bit()),
            1 => self.register.route.modify(|_, w| w.cc1pen().clear_bit()),
            2 => self.register.route.modify(|_, w| w.cc2pen().clear_bit()),
            _ => panic!("Nonexistent channel"),
        }
    }

    #[cfg(feature = "chip-efr32xg1")]
    fn disable(&mut self, channel: Self::Channel) {
        match channel {
            0 => self.register.routepen.modify(|_, w| w.cc0pen().clear_bit()),
            1 => self.register.routepen.modify(|_, w| w.cc1pen().clear_bit()),
            2 => self.register.routepen.modify(|_, w| w.cc2pen().clear_bit()),
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
        self.register.top.read().top().bits()
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
        match channel {
            0 => self.register.cc0_ccv.read().ccv().bits(),
            1 => self.register.cc1_ccv.read().ccv().bits(),
            2 => self.register.cc2_ccv.read().ccv().bits(),
            _ => panic!("Nonexistent channel"),
        }
    }
}

impl $TimerN {
    /// Configure whether the output is inverted (false: low duty cycle means line is low most of
    /// the time, true: low duty cycle means line is high most of the time).
    ///
    /// While this can largely be adjusted for by setting the duty to max-n instead of n, inverted
    /// also means that the output is high during program interruptions (eg. debugging).
    pub fn set_inverted(&mut self, channel: <Self as embedded_hal::Pwm>::Channel, inverted: bool) {
        match channel {
            0 => self.register.cc0_ctrl.modify(|_, w| w.outinv().bit(inverted)),
            1 => self.register.cc1_ctrl.modify(|_, w| w.outinv().bit(inverted)),
            2 => self.register.cc2_ctrl.modify(|_, w| w.outinv().bit(inverted)),
            _ => panic!("Nonexistent channel"),
        }
    }

    /// Configure the top value for this channel.
    ///
    /// As this limits the duty cycle, it can be read back using the PWM method get_max_duty().
    pub fn set_top(&mut self, top: u16) {
        self.register.top.modify(|_, w| unsafe { w.top().bits(top) });
    }

    // The following functions mimic the cortex_m::peripheral::NVIC interrupt controller, as they
    // behave like a sub-controller for a particular interrupt. FIXME: Generalize this over all
    // EFM32 devices with their _IEN/_IF/_IFS/_IFC registers

    pub fn interrupt_enable(&mut self, interrupt: InterruptFlag) {
        self.register.ien.modify(|r, w| unsafe { w.bits(interrupt.bits() | r.bits()) });
    }

    pub fn interrupt_is_pending(interrupt: InterruptFlag) -> bool {
        let reg = unsafe { &*registers::$TIMERn::ptr() };
        reg.if_.read().bits() & interrupt.bits() != 0
    }

    pub fn interrupt_unpend(interrupt: InterruptFlag) {
        unsafe {
            let reg = &*registers::$TIMERn::ptr();
            reg.ifc.write(|w|  w.bits(interrupt.bits()) );
        }
    }

    // FIXME this should definitely be type state
    pub fn enable_outputcompare(&mut self, channel: i32) {
        match channel {
            0 => self.register.cc0_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc0_ctrl::MODEW::OUTPUTCOMPARE)),
            1 => self.register.cc1_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc1_ctrl::MODEW::OUTPUTCOMPARE)),
            2 => self.register.cc2_ctrl.modify(|_, w| w.mode().variant(registers::$timerN::cc2_ctrl::MODEW::OUTPUTCOMPARE)),
            _ => panic!("Nonexistent channel"),
        }
    }


    /// Do something else with the registers; this is marked unsafe because one might do things
    /// like re-route pins
    pub unsafe fn with_registers<T>(&mut self, action: impl FnOnce(&mut registers::$TIMERn) ->T) -> T {
        action(&mut self.register)
    }
}

    }
}

/// Timer interrupt flags
///
/// Each value represents a particular interrupt flag that is available for enabling, setting and
/// clearing in all timers.
///
/// These are implemented explicitly rather than re-using the register block's individual types, as
/// not only those are duplicate across the timers (a common occurrence in svd2rust crates), but
/// even over all interrupt registers of a timer. Implementing them as one bakes in the assumption
/// that the same flags that can be enabled can also be set or cleared.
#[derive(Copy, Clone)]
pub enum InterruptFlag {
    /// Overflow
    OF = 1,
    /// Underflow
    UF = 2,
    /// CC Channel 0
    CC0 = 16,
    /// CC Channel 1
    CC1 = 32,
    /// CC Channel 2
    CC2 = 64,
    /// CC Channel 0 Input Capture Buffer Overflow
    ICBOF0 = 256,
    /// CC Channel 1 Input Capture Buffer Overflow
    ICBOF1 = 512,
    /// CC Channel 2 Input Capture Buffer Overflow
    ICBOF2 = 1024,
}

impl InterruptFlag {
    const fn bits(&self) -> u32 { *self as u32 }
}

timer!(TIMER0, TIMER0Clk, Timer0, timer0);
timer!(TIMER1, TIMER1Clk, Timer1, timer1);
#[cfg(feature = "_has_timer2")]
timer!(TIMER2, TIMER2Clk, Timer2, timer2);

use gpio;

#[cfg(feature = "chip-efm32gg")]
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

#[cfg(feature = "chip-efm32gg")]
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
