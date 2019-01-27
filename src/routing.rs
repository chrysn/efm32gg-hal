/// These submodules contain encoded in them which route locations of peripherals correspond to
/// which GPIO pins.
///
/// This whole module is private as it only contains the (thus sealed) traits by which pins declare
/// their routing sources.

/// Indicates that a given pin is a valid routing location for a given peripheral and function
pub trait HasLocForFunction<P, F> {
    unsafe fn configure();
}

mod per_function;
mod per_peripheral;

#[cfg(feature = "chip-efr32xg1")]
mod efr32xg1;
#[cfg(feature = "chip-efm32gg")]
mod efm32gg;
