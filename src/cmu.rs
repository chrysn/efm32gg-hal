//! CMU (Clock Management Unit)
//!
//! This first implementation tries to anticipate what can later be done with the clocks, but
//! only implements what is essential do have working delays.
//!
//! The workflow is probably going to be:
//! * Take the SVD-derived peripheral (CMU implementing CMUExt), consume it into a Cmu that we can
//!   actually work on.
//! * Split (or freeze?) the Cmu into all the configured Clocks that are now objects which know
//!   their timing and cant' be changed.
//!   * This splitting process may become more-phased later, such that one could freeze LFXO based
//!     clocks when they are up, and still build HFXO based clocks on demand.
//! * Timers, delays, but also timed peripherals (eg. UARTs) can then be built from those clocks;
//!   not sure whether they'll need to consume them, it may suffice to get a long-lived reference
//!   to them.
//!     * If clocks need to be counsumed, the peripherals could still "spit them out" again, and if
//!       one owns all the parts of a frozen clock, one might change that one again. (Esp. if we
//!       want to model deep sleep states correcly; starting a sleep state that disables HF clocks
//!       should require having a writable reference to that clock.)

use registers;

use time_util::Hertz;

pub trait CMUExt {
    fn constrain(self) -> Cmu;
}

impl CMUExt for registers::CMU {
    fn constrain(self) -> Cmu {
        Cmu { _private: () }
    }
}

pub struct Cmu {
    // Just make sure this can't be created from outside; becomes obsolete when there are other
    // non-pub fields.
    _private: (),
}

pub trait FrozenClock {
    fn get_frequency(&self) -> Hertz;
}

pub struct Clocks {
    pub hfcoreclk: HFCoreClk,
}

impl Cmu {
    pub fn split(self) -> Clocks {
        Clocks {
            hfcoreclk: HFCoreClk { _private: () },
        }
    }
}

pub struct HFCoreClk {
    _private: (),
}

impl FrozenClock for HFCoreClk {
    fn get_frequency(&self) -> Hertz {
        // FIXME: This assumes that nothing has been changed since the reset; later we'll *want*
        // that to change.
        Hertz(14_000_000)
    }
}
