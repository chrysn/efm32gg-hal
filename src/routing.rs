/// These submodules contain encoded in them which route locations of peripherals correspond to
/// which GPIO pins.
///
/// This whole module is private as it only contains the (thus sealed) traits by which pins declare
/// their routing sources.

/// Indicates that a given pin is a valid routing location for a given peripheral and function
pub trait HasLocForFunction<P, F> {
    /// Configure and enable the routed pin; what this means in detail depends on the main caller
    /// (eg. TimerChannel::route).
    unsafe fn configure();
    /// Disable the routed pin
    unsafe fn deconfigure();
}

mod per_function;
mod per_peripheral;

#[cfg(feature = "chip-efm32gg")]
mod efm32gg;
#[cfg(feature = "chip-efm32gg11b820")]
mod efm32gg11;
#[cfg(feature = "chip-efr32xg1")]
mod efr32xg1;
