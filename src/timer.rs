//! TIMER (Timer/Counter peripheral)
//!
//! This module exposes some features of the EFM32 timer/counter peripheral; most notably, it
//! allows easy configuration of PWM pins.

use core::marker::PhantomData;

use registers;
use crate::cmu;

pub trait TimerExt<Clk, Timer> {
    fn with_clock(self, clock: Clk) -> Timer;
}

/// Marker type for timer channels, signifying they're a CC channel 0 of whichever timer
pub struct Channel0 {}
/// Marker type for timer channels, signifying they're a CC channel 1 of whichever timer
pub struct Channel1 {}
/// Marker type for timer channels, signifying they're a CC channel 2 of whichever timer
pub struct Channel2 {}

pub struct Channels<C0, C1, C2> {
    pub channel0: C0,
    pub channel1: C1,
    pub channel2: C2,
}

/// Individual channel of a timer, accessible through a timer's .split() method.
pub struct TimerChannel<Timer, Channel> {
    _phantom: PhantomData<(Timer, Channel)>,
}

impl<T, C> TimerChannel<T, C> {
    /// Configure a route from the timer channel to the given output pin.
    ///
    /// This changes the timer's mode to PWM, as that's the only mode currently in use.
    ///
    /// The pin will not reliably be enabled or disabled upon initialization.
    pub fn route<P>(self, pin: P) -> RoutedTimerChannel<T, C, P> where
        P: crate::routing::HasLocForFunction<T, C>,
    {
        unsafe { P::configure() };
        RoutedTimerChannel {
            channel: self,
            pin
        }
    }
}

impl<T, C, P> RoutedTimerChannel<T, C, P> where
    P: crate::routing::HasLocForFunction<T, C>,
    TimerChannel<T, C>: embedded_hal::PwmPin,
{
    /// Free the routed timer channel's GPIO pin.
    ///
    /// This is the inverse of `TimerChannel::route(timerchannel, pin)`, but does not change the
    /// channel's route (as it's immaterial once the pin is disabled), and leaves the channel's
    /// mode at PWM.
    ///
    /// It does, however, disable the channel, for otherwise the pin would stay influenced by a now
    /// unrelated peripheral.
    pub fn unroute(mut self) -> (TimerChannel<T, C>, P) {
        use embedded_hal::PwmPin;

        self.channel.disable();

        (self.channel, self.pin)
    }
}

pub struct RoutedTimerChannel<Timer, Channel, Pin> {
    pub(crate) pin: Pin,
    pub(crate) channel: TimerChannel<Timer, Channel>
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

    pub fn start(&mut self) {
        self.register.cmd.write(|w| w.start().bit(true));
    }

    pub fn split(self) -> Channels<
        TimerChannel<$TimerN, Channel0>,
        TimerChannel<$TimerN, Channel1>,
        TimerChannel<$TimerN, Channel2>,
    > {
        Channels {
            channel0: TimerChannel { _phantom: PhantomData },
            channel1: TimerChannel { _phantom: PhantomData },
            channel2: TimerChannel { _phantom: PhantomData },
        }
    }


    /// Do something else with the registers; this is marked unsafe because one might do things
    /// like re-route pins
    pub unsafe fn with_registers<T>(&mut self, action: impl FnOnce(&mut registers::$TIMERn) ->T) -> T {
        action(&mut self.register)
    }
}

// Needs to be actually repeated over the channels because the channel structs can't, for example,
// produce a .cc0_ctrl.modify() artifact because there is nothing to be generic over.

timerchannel!($TIMERn, $TimerN, $timerN, Channel0, cc0pen, cc0_ctrl, cc0_ccv);
timerchannel!($TIMERn, $TimerN, $timerN, Channel1, cc1pen, cc1_ctrl, cc1_ccv);
timerchannel!($TIMERn, $TimerN, $timerN, Channel2, cc2pen, cc2_ctrl, cc2_ccv);

    }
}

macro_rules! timerchannel {
    ($TIMERn: ident, $TimerN: ident, $timerN: ident, $ChannelX: ident, $ccXpen: ident, $ccX_ctrl: ident, $ccX_ccv: ident) => {

impl TimerChannel<$TimerN, $ChannelX> {
    /// Get a pointer to the underlying timer's peripheral block.
    ///
    /// Accessing that is safe only to the CCx registers of this block, as those are exclusive to
    /// this struct which by construction gets only created once.
    pub(crate) fn register() -> *mut registers::$timerN::RegisterBlock {
        registers::$TIMERn::ptr() as *mut _
    }
}

impl<P> RoutedTimerChannel<$TimerN, $ChannelX, P> {
    // Like TimerChannel::register()
    fn register(&self) -> *mut registers::$timerN::RegisterBlock {
        registers::$TIMERn::ptr() as *mut _
    }

    /// Configure whether the output channel is inverted (false: low duty cycle means line is low
    /// most of the time, true: low duty cycle means line is high most of the time).
    ///
    /// While this can largely be adjusted for by setting the duty to max-n instead of n, inverted
    /// also means that the output is high during program interruptions (eg. debugging).
    pub fn set_inverted(&mut self, inverted: bool) {
        // Unsafe: OK because it's a CCx register (see .register())
        unsafe { &mut *self.register() }.$ccX_ctrl.modify(|_, w| w.outinv().bit(inverted));
    }
}

impl<P> embedded_hal::PwmPin for RoutedTimerChannel<$TimerN, $ChannelX, P> {
    type Duty = u16; // FIXME check the extreme behaviors

    fn enable(&mut self) {
        // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
        cortex_m::interrupt::free(|_| {
            #[cfg(not(feature = "_routing_per_function"))]
            {
                unsafe { &mut *self.register() }.route.modify(|_, w| w.$ccXpen().set_bit());
            }
            #[cfg(feature = "_routing_per_function")]
            {
                unsafe { &mut *self.register() }.routepen.modify(|_, w| w.$ccXpen().set_bit());
            }
        });
    }
    fn disable(&mut self) {
        // FIXME https://github.com/chrysn/efm32gg-hal/issues/1
        cortex_m::interrupt::free(|_| {
            #[cfg(not(feature = "_routing_per_function"))]
            {
                unsafe { &mut *self.register() }.route.modify(|_, w| w.$ccXpen().clear_bit());
            }
            #[cfg(feature = "_routing_per_function")]
            {
                unsafe { &mut *self.register() }.routepen.modify(|_, w| w.$ccXpen().clear_bit());
            }
        });
    }

    fn get_duty(&self) -> Self::Duty {
        // Unsafe: Accessign a CCx register, see .register()
        unsafe { &*self.register() }.$ccX_ccv.read().ccv().bits() as Self::Duty
    }
    fn get_max_duty(&self) -> Self::Duty {
        // Unsafe: Read-only access to a register shared among the pins and thus not written to by
        // anyone else (besides, it's a guaranteed atomic read)
        unsafe { &*self.register() }.top.read().bits() as Self::Duty
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        // Unsafe: OK because it's a CC0 register (see .register())
        unsafe { &mut *self.register() }.$ccX_ccv.modify(|_, w| unsafe { w.ccv().bits(duty) })
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
