use core::marker::PhantomData;

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
///     Output<WithFilter<WiredAnd<Normal, Floating>>>
///     Output<WithFilter<WiredAnd<Normal, PullUp>>>
///     Output<WiredAnd<Alternate, Floating>>
///     Output<WiredAnd<Alternate, PullUp>>
///     Output<WithFilter<WiredAnd<Alternate, Floating>>>
///     Output<WithFilter<WiredAnd<Alternate, PullUp>>>
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
pub struct OpenDrain<MODE, STATE> {
    _private0: PhantomData<MODE>,
    _private1: PhantomData<STATE>,
}

pub struct WithFilter<STATE> {
    _private: PhantomData<STATE>,
}

pub type WiredOr<STATE> = OpenSource<STATE>;
pub type WiredAnd<MODE, STATE> = OpenDrain<MODE, STATE>;

pub struct Normal;
pub struct Alternate;
