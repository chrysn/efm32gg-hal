#![no_std]

extern crate cortex_m;
extern crate embedded_hal;

#[cfg(feature="chip-efm32gg")]
extern crate efm32gg990 as registers;
#[cfg(feature="chip-efr32xg1")]
extern crate efr32xg1 as registers;

pub mod time_util;

pub mod gpio;
pub mod i2c;
pub mod cmu;
pub mod systick;

mod bitband;
