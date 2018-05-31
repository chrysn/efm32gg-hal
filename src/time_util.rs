//! Time helpers
//!
//! These are copied from stm32f30x-hal's time helpers (from
//! <https://github.com/japaric/stm32f30x-hal/blob/master/src/time.rs>) on demand.

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);
