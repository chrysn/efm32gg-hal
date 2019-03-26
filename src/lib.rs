#![no_std]

extern crate cortex_m;
extern crate embedded_hal;

#[cfg(feature = "chip-efm32gg")]
extern crate efm32gg990 as registers;

#[cfg(feature = "chip-efr32xg1")]
extern crate efr32xg1 as registers;

#[cfg(feature = "chip-efm32hg")]
extern crate efm32hg as registers;

pub mod time_util;

pub mod cmu;
pub mod gpio;

// Right now that's implemented only there, and does not have the internal cfgs yet to run on
// efm32gg as well
#[cfg(feature = "chip-efr32xg1")]
pub mod i2c;

pub mod systick;
pub mod timer;

mod bitband;

mod routing;
