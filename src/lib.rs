#![no_std]

extern crate cortex_m;
extern crate efm32gg990;
extern crate embedded_hal;

pub mod time_util;

pub mod gpio;
pub mod cmu;
pub mod systick;

mod bitband;
