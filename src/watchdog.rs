//! Watchog
//!
//! This module exposes feeding and disabling of the EFM32 watchdog peripheral.
//! `let mut watchdog = Watchdog::new(p.WDOG);` then `watchdog.disable();`
//! or `watchdog.feed();`.

use embedded_hal::watchdog;
use registers;

#[cfg(feature = "unproven")]
pub struct Watchdog {
    wdog: registers::WDOG,
}

// TODO: configuration of new watchdog
#[cfg(feature = "unproven")]
#[allow(dead_code)]
impl Watchdog {
    pub fn new(wdog: registers::WDOG) -> Self {
        Self { wdog }
    }

    fn free(self) -> registers::WDOG {
        self.wdog
    }
}

#[cfg(feature = "unproven")]
impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        unsafe {
            self.wdog.cmd.write(|w| w.bits(1));
        }
    }
}

#[cfg(feature = "unproven")]
impl watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        unsafe {
            self.wdog.ctrl.write(|w| w.bits(0));
        }
    }
}
