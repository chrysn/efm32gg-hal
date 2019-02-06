use super::Pins;
use crate::cmu;
use embedded_hal::digital;

/// State type for pin with disabled input.
/// In disabled state output pin can be in either
/// floating state (dout 0), or pulled up (dout 1)
pub struct Disabled<TYPE> {
    _private: PhantomData<TYPE>,
}

/// In efr32 chip series, casting pin into Output will set DINDIS register to 1.
/// There is no Output to Input conversion in any direction.
/// To enable both Input and Output, cast to `Input<Output<_state_>>` type from Disabled.
/// See Input type below.
/// List of possible Output mode:
///     Output<PushPull<Normal>>
///     Output<PushPull<Alternate>>
///     Output<WiredOr<Floating>>
///     Output<WiredOr<PullDown>>
///     Output<WiredAnd<Normal, Floating>>
///     Output<WiredAnd<Normal, PullUp>>
///     Output<WiredAndFilter<Normal, Floating>>
///     Output<WiredAndFilter<Normal, PullUp>>
///     Output<WiredAnd<Alternate, Floating>>
///     Output<WiredAnd<Alternate, PullUp>>
///     Output<WiredAndFilter<Alternate, Floating>>
///     Output<WiredAndFilter<Alternate, PullUp>>
pub struct Output<TYPE> {
    _private: PhantomData<TYPE>,
}

/// List of possible input mode:
///     Input<Floating>
///     Input<PullUp>
///     Input<PullDown>
///     Input<WithFilter<Floating>>
///     Input<WithFilter<PullDown>>
///     Input<WithFilter<PullUp>>
///
/// This mode, will also has Output implemented, see Output type above:
///     Input<_all output modes_>
pub struct Input<TYPE> {
    _private: PhantomData<TYPE>,
}

/// Pin states.
pub struct Floating;
pub struct PullUp;
pub struct PullDown;

/// Output mode.
///
/// PushPull can have:
///     PushPull<Normal>    // for efr32, Normal config in ctrl also can be configured separately.
///     PushPull<Alternate> // for efm32, Alternate config only change its current drive mode.
pub struct PushPull<CFG> {
    _private: PhantomData<CFG>,
}

/// OpenSource (wired "or") pin is set to be driven high.
/// Its state can be Floating or PullDown.
pub struct OpenSource<STATE> {
    _private: PhantomData<STATE>,
}

/// OpenDrain (wired "and") pin is set to be driven low.
/// Its state can be Floating or PullUp.
pub struct OpenDrain<STATE> {
    _private: PhantomData<STATE>,
}

type WiredOr = OpenSource;
type WiredAnd = OpenDrain;

pub struct Normal;
pub struct Alternate;

pub enum DriveMode {
    High,

    Lowest,

    #[cfg(not(feature = "chip-efr32xg1"))]
    Standard,

    #[cfg(not(feature = "chip-efr32xg1"))]
    Low,
}

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
