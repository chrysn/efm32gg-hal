//! TIMER (Timer/Counter peripheral)
//!
//! This module exposes some features of the EFM32 timer/counter peripheral; most notably, it
//! allows easy configuration of PWM pins.

use core::marker::PhantomData;

pub trait TimerExt<Clk, Timer> {
    fn with_clock(self, clock: Clk) -> Timer;
}

/// Marker type for timer channels, signifying they're a CC channel 0 of whichever timer
pub struct Channel0 {}
/// Marker type for timer channels, signifying they're a CC channel 1 of whichever timer
pub struct Channel1 {}
/// Marker type for timer channels, signifying they're a CC channel 2 of whichever timer
pub struct Channel2 {}


/// Individual channel of a timer, accessible through a timer's .split() method.
pub struct TimerChannel<Timer, Channel> {
    _phantom: PhantomData<(Timer, Channel)>,
}

impl<T, C> TimerChannel<T, C> {
    /// Configure a route from the timer channel to the given output pin.
    ///
    /// This routes the pin; whether it is set to Off or Pwm is managed though Pwm::enable/disable.
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
        unsafe { P::deconfigure() };

        (self.channel, self.pin)
    }
}

pub struct RoutedTimerChannel<Timer, Channel, Pin> {
    pub(crate) pin: Pin,
    pub(crate) channel: TimerChannel<Timer, Channel>
}

macro_rules! timer {
    ($TIMERn: ident, $TIMERnClk: ident, $TimerN: ident, $timerN: ident, $channel: tt) => {

mod $timerN {

use super::*;

use crate::cmu;
use registers;

impl TimerExt<cmu::$TIMERnClk, $TimerN> for registers::$TIMERn {
    fn with_clock(self, mut clock: cmu::$TIMERnClk) -> $TimerN {
        clock.enable();
        $TimerN { register: self, clock }
    }
}

pub struct $TimerN {
    pub(crate) register: registers::$TIMERn,
    clock: cmu::$TIMERnClk,
}

impl $TimerN {
    /// Configure the top value for this timer.
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

    #[cfg(not(feature = "_routing_per_function"))]
    /// Preconfigure a pin route on the whole device
    ///
    /// This has no immediate effect as (when a full $TimerN is still available) all its output
    /// pins are disabled, but prepares for when the individual output channels are `.route()`d.
    ///
    /// This is only present for the original EFM32 devices (up to Wonder Gecko) that have per-peripheral
    /// routing. In those devices, routing needs to be put in place while the whole peripheral is
    /// still mutable -- and the later routing functions only assert on that register's state.
    pub fn preroute(&mut self, route: registers::$timerN::route::LOCATIONW) {
        self.register.route.modify(|_, w| w.location().variant(route))
    }

    /// Dissect this timer into its various channels, consuming the timer.
    ///
    /// The returning struct is non-public intentionally, as it is expected to grow when additional
    /// channels are implemented. Channels can be moved out of this struct as `.channel[0-2]`
    /// attributes.
    pub fn split(self) -> Channels {
        Channels {
            channel0: TimerChannel { _phantom: PhantomData },
            channel1: TimerChannel { _phantom: PhantomData },
            channel2: TimerChannel { _phantom: PhantomData },
        }
    }
}

/// The channels available on this particular timer. This struct is expected to grow, so don't
/// destructure it but rather move out of it what you need.
pub struct Channels {
    pub channel0: TimerChannel<$TimerN, Channel0>,
    pub channel1: TimerChannel<$TimerN, Channel1>,
    pub channel2: TimerChannel<$TimerN, Channel2>,
}

// Needs to be actually repeated over the channels because the channel structs can't, for example,
// produce a .cc0_ctrl.modify() artifact because there is nothing to be generic over.

timerchannel!($TIMERn, $TimerN, $timerN, Channel0, cc0_ctrl, cc0_ccvb);
timerchannel!($TIMERn, $TimerN, $timerN, Channel1, cc1_ctrl, cc1_ccvb);
timerchannel!($TIMERn, $TimerN, $timerN, Channel2, cc2_ctrl, cc2_ccvb);

}

pub use $timerN::$TimerN;

    }
}

macro_rules! timerchannel {
    ($TIMERn: ident, $TimerN: ident, $timerN: ident, $ChannelX: ident, $ccX_ctrl: ident, $ccX_ccvb: ident) => {

impl TimerChannel<$TimerN, $ChannelX> {
    /// Get a pointer to the underlying timer's peripheral block.
    ///
    /// Accessing that is safe only to the CCx registers of this block, as those are exclusive to
    /// this struct which by construction gets only created once.
    pub(crate) fn register() -> *mut registers::$timerN::RegisterBlock {
        registers::$TIMERn::ptr() as *mut _
    }

    fn set_mode(&mut self, mode: ChannelMode) {
        // Unsafe: OK because it's a CC0 register (see .register())
        unsafe { &mut *Self::register() }.$ccX_ctrl.modify(|_, w| match mode {
            ChannelMode::Off => w.mode().off(),
            ChannelMode::InputCapture => w.mode().inputcapture(),
            ChannelMode::OutputCompare => w.mode().outputcompare(),
            ChannelMode::Pwm => w.mode().pwm(),
        });
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
    ///
    /// (Note that this *currently* does not affect the state of a disabled pin. It might become
    /// convenient at a later time to change the enable/disable mechanism to something that *does*
    /// respect the set_inverted setting.)
    pub fn set_inverted(&mut self, inverted: bool) {
        // Unsafe: OK because it's a CCx register (see .register())
        unsafe { &mut *self.register() }.$ccX_ctrl.modify(|_, w| w.outinv().bit(inverted));
    }
}

impl<P> embedded_hal::PwmPin for RoutedTimerChannel<$TimerN, $ChannelX, P> {
    type Duty = u16; // FIXME check the extreme behaviors

    fn enable(&mut self) {
        self.channel.set_mode(ChannelMode::Pwm);
    }
    fn disable(&mut self) {
        self.channel.set_mode(ChannelMode::Off);
    }

    fn get_duty(&self) -> Self::Duty {
        // Unsafe: Accessign a CCx register, see .register()
        unsafe { &*self.register() }.$ccX_ccvb.read().ccvb().bits() as Self::Duty
    }
    fn get_max_duty(&self) -> Self::Duty {
        // Unsafe: Read-only access to a register shared among the pins and thus not written to by
        // anyone else (besides, it's a guaranteed atomic read)
        unsafe { &*self.register() }.top.read().bits() as Self::Duty
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        // Unsafe: OK because it's a CC0 register (see .register())
        unsafe { &mut *self.register() }.$ccX_ccvb.modify(|_, w| unsafe { w.ccvb().bits(duty) })
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

/// Helper for TimerChannel.set_mode
enum ChannelMode {
    Off,
    InputCapture,
    OutputCompare,
    Pwm
}

timer!(TIMER0, TIMER0Clk, Timer0, timer0, [
       (Channel0, cc0_ctrl, cc0_ccvb),
       (Channel1, cc1_ctrl, cc1_ccvb),
       (Channel2, cc2_ctrl, cc2_ccvb),
    ]);
timer!(TIMER1, TIMER1Clk, Timer1, timer1, [
       (Channel0, cc0_ctrl, cc0_ccvb),
       (Channel1, cc1_ctrl, cc1_ccvb),
       (Channel2, cc2_ctrl, cc2_ccvb),
    ]);
#[cfg(feature = "_has_timer2")]
timer!(TIMER2, TIMER2Clk, Timer2, timer2, [
       (Channel0, cc0_ctrl, cc0_ccvb),
       (Channel1, cc1_ctrl, cc1_ccvb),
       (Channel2, cc2_ctrl, cc2_ccvb),
    ]);
#[cfg(feature = "_has_timer3")]
timer!(TIMER3, TIMER3Clk, Timer3, timer3, [
       (Channel0, cc0_ctrl, cc0_ccvb),
       (Channel1, cc1_ctrl, cc1_ccvb),
       (Channel2, cc2_ctrl, cc2_ccvb),
    ]);
