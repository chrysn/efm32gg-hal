use super::Pins;
use crate::cmu;
use embedded_hal::digital;

pub struct Disabled;
pub struct Output;
pub struct Input;

pub trait GPIOExt {
    fn split(self, gpioclk: cmu::GPIOClk) -> Pins;
}

/// A trait pertinent to a single GPIO pin; this trait exposes all the functionality that is not
/// exposed via the embedded_hal::digital traits.
///
/// Given that different modes of operation make sense with different APIs (a pin configured for
/// output can be driven, one for input can be queried), the trait is not implemented by a single
/// struct for a pin but qualified by a parameterized struct whose expressions implement different
/// traits; for example, PA0<T> implements EFM32Pin for all T, and configuring it as an output pin
/// with `.as_output()` makes it into a PA0<Output> which implements digital::OutputPin.
///
/// Currently, not all information about a pin is encoded in its type (For example, as_output and
/// as_opendrain produce the same type), that may change just by adding an InputOutput
/// configuration state that has implements both input and output traits (as it makes sense to both
/// write to and read an open drain pin), or may even go as far as to encode the full pin
/// configuration.
pub trait EFM32Pin {
    type Disabled;
    #[cfg(not(feature = "unproven"))]
    type Output: digital::OutputPin;
    #[cfg(feature = "unproven")]
    type Output: digital::OutputPin + digital::StatefulOutputPin;
    type Input: digital::InputPin;

    /// Convert the pin into an output (push-pull) pin. The original pin, however configured, is consumed, the
    /// hardware configuration changed to drive high or low, and returned as a pin that implements
    /// embedded_hal::digital::OutputPin.
    fn as_output(self: Self) -> Self::Output;

    /// Convert the pin into an open drain (wired "and") pin. The original pin, however configured,
    /// is consumed, the hardware configuration changed to only drive low, and returned as a pin
    /// that implements embedded_hal::digital::OutputPin and embedded_hal::digital::InputPin.
    fn as_opendrain(self: Self) -> Self::Output;

    /// Convert the pin into an input pin. The original pin, however configured, is consumed, the
    /// hardware configuration changed to input with no pull-up- or down resistors, and returned as
    /// a pin that implements embedded_hal::digital::InputPin.
    fn as_input(self: Self) -> Self::Input;
}
