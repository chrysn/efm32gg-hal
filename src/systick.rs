//! The Core-M SysTick peripheral
//!
//! This is implemented in efm32gg-hal and not a generic Cortex-M HAL because the relevant
//! functionality depends not only on the peripheral but also on the (EFM32) clock source that
//! feeds it.
//!
//! This looks quite a bit like the stm32f30x-hal delays.rs
//! <https://github.com/japaric/stm32f30x-hal/blob/master/src/delay.rs> -- I tried out of curiosity
//! to come up with an own solution, and it turns out thsi is how it needs to be done
//!
//! FIXME: factor out the common parts (which should be everything except the actual numbers for
//! the clock frequency depending on the SystClkSource) into ... core-m-hal?

use crate::cmu::{FrozenClock, HFCoreClk};
use cortex_m;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub trait SystickExt {
    fn constrain(self) -> Systick;
}

impl SystickExt for cortex_m::peripheral::SYST {
    fn constrain(self) -> Systick {
        Systick {
            registerblock: self,
        }
    }
}

pub struct Systick {
    // We could use a zero-sized abstraction here like we do for GPIO pins, but it's internal
    // anyway and I don't care about those 4 byte right now; feel free to bend it.
    registerblock: cortex_m::peripheral::SYST,
}

// We might need to introduce a type parameter later to say whether clock is HFCoreClk or RTC, and
// have individual new methods that set CLKSOURCE.
pub struct SystickDelay {
    systick: Systick,
    clock: HFCoreClk,
}

impl SystickDelay {
    pub fn new(mut systick: Systick, clock: HFCoreClk) -> Self {
        systick
            .registerblock
            .set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);

        SystickDelay { systick, clock }
    }
}

impl<UXX> DelayUs<UXX> for SystickDelay
where
    UXX: Into<u32>,
{
    fn delay_us(&mut self, us: UXX) {
        // FIXME this assumes clock rate is in whole MHz, which usually holds.
        let factor = self.clock.get_frequency().0 / 1_000_000;
        // Just trigger the assertion...
        let ticks = factor.checked_mul(us.into()).unwrap_or(1 << 24);

        // FIXME: If we can show that all the above calculation can be done in LTO, then I'd be
        // much more comfortable adding logic that goes into loops for sleeps exceeding one systick
        // wrap (which is about 2s on typical 14MHz devices).
        assert!(ticks < (1 << 24));

        self.systick.registerblock.set_reload(ticks);
        self.systick.registerblock.clear_current();
        self.systick.registerblock.enable_counter();

        while !self.systick.registerblock.has_wrapped() {}
        self.systick.registerblock.disable_counter();
    }
}

// Limited to u16 because waiting for 2**16 or more ms already exceeds what the DelayUs
// implementation can do even on a 1MHz clock, and lower clock frequencies can't be expressed
// anyway in that implementation.
impl<UXX> DelayMs<UXX> for SystickDelay
where
    UXX: Into<u16>,
{
    fn delay_ms(&mut self, ms: UXX) {
        let ms: u16 = ms.into();
        let ms: u32 = ms.into();
        self.delay_us(ms * 1000);
    }
}
