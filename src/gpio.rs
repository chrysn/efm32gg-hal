use efm32gg990;
use embedded_hal::digital;

use core::marker::PhantomData;

pub struct Disabled {}
pub struct Output {}
pub struct Input {}

fn sneak_into_gpio() -> &'static efm32gg990::gpio::RegisterBlock {
        unsafe { &*efm32gg990::GPIO::ptr() }
}

macro_rules! gpio {
    ([$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_doutset:ident, $px_doutclr:ident, $modei:ident, $px_modehl:ident),)+]) => {

        $(
            pub struct $PXi<Mode> {
                _mode: PhantomData<Mode>,
            }

            impl digital::OutputPin for $PXi<Output> {
                fn set_low(self: &mut Self) {
                    let gpio = sneak_into_gpio();
                    gpio.$px_doutclr.write(|w| unsafe { w.bits(1 << $i) });
                }

                fn set_high(self: &mut Self) {
                    let gpio = sneak_into_gpio();
                    gpio.$px_doutset.write(|w| unsafe { w.bits(1 << $i) });
                }
            }
            #[cfg(feature = "unproven")]
            impl digital::InputPin for $PXi<Input> {
                fn is_low(self: &Self) -> bool {
                    let gpio = sneak_into_gpio();
                    gpio.$px_din.read().bits() & (1 << $i) == 0
                }

                fn is_high(self: &Self) -> bool {
                    !self.is_low()
                }
            }

            impl<Mode> $PXi<Mode> {
                pub fn as_output(self: Self) -> $PXi<Output> {
                    let gpio = sneak_into_gpio();
                    gpio.$px_modehl.modify(|_, w| w.$modei().pushpull());

                    $PXi { _mode: PhantomData }
                }
            }

            impl<Mode> $PXi<Mode> {
                pub fn as_input(self: Self) -> $PXi<Input> {
                    let gpio = sneak_into_gpio();
                    gpio.$px_modehl.modify(|_, w| w.$modei().input());

                    $PXi { _mode: PhantomData }
                }
            }
        )+

        pub struct Pins {
            $(
                pub $pxi: $PXi<Disabled>,
            )+
        }

        pub fn split(_comsumed_peripheral: efm32gg990::GPIO) -> Pins {
            Pins {
                $(
                    $pxi: $PXi { _mode: PhantomData },
                )+
            }
        }
    }
}



gpio!([
    PA0:  (pa0,  0,  pa_din, pa_doutset, pa_doutclr, mode0,  pa_model),
    PA1:  (pa1,  1,  pa_din, pa_doutset, pa_doutclr, mode1,  pa_model),
    PA2:  (pa2,  2,  pa_din, pa_doutset, pa_doutclr, mode2,  pa_model),
    PA3:  (pa3,  3,  pa_din, pa_doutset, pa_doutclr, mode3,  pa_model),
    PA4:  (pa4,  4,  pa_din, pa_doutset, pa_doutclr, mode4,  pa_model),
    PA5:  (pa5,  5,  pa_din, pa_doutset, pa_doutclr, mode5,  pa_model),
    PA6:  (pa6,  6,  pa_din, pa_doutset, pa_doutclr, mode6,  pa_model),
    PA7:  (pa7,  7,  pa_din, pa_doutset, pa_doutclr, mode7,  pa_model),
    PA8:  (pa8,  8,  pa_din, pa_doutset, pa_doutclr, mode8,  pa_modeh),
    PA9:  (pa9,  9,  pa_din, pa_doutset, pa_doutclr, mode9,  pa_modeh),
    PA10: (pa10, 10, pa_din, pa_doutset, pa_doutclr, mode10, pa_modeh),
    PA11: (pa11, 11, pa_din, pa_doutset, pa_doutclr, mode11, pa_modeh),
    PA12: (pa12, 12, pa_din, pa_doutset, pa_doutclr, mode12, pa_modeh),
    PA13: (pa13, 13, pa_din, pa_doutset, pa_doutclr, mode13, pa_modeh),
    PA14: (pa14, 14, pa_din, pa_doutset, pa_doutclr, mode14, pa_modeh),
    PA15: (pa15, 15, pa_din, pa_doutset, pa_doutclr, mode15, pa_modeh),
    PB0:  (pb0,  0,  pb_din, pb_doutset, pb_doutclr, mode0,  pb_model),
    PB1:  (pb1,  1,  pb_din, pb_doutset, pb_doutclr, mode1,  pb_model),
    PB2:  (pb2,  2,  pb_din, pb_doutset, pb_doutclr, mode2,  pb_model),
    PB3:  (pb3,  3,  pb_din, pb_doutset, pb_doutclr, mode3,  pb_model),
    PB4:  (pb4,  4,  pb_din, pb_doutset, pb_doutclr, mode4,  pb_model),
    PB5:  (pb5,  5,  pb_din, pb_doutset, pb_doutclr, mode5,  pb_model),
    PB6:  (pb6,  6,  pb_din, pb_doutset, pb_doutclr, mode6,  pb_model),
    PB7:  (pb7,  7,  pb_din, pb_doutset, pb_doutclr, mode7,  pb_model),
    PB8:  (pb8,  8,  pb_din, pb_doutset, pb_doutclr, mode8,  pb_modeh),
    PB9:  (pb9,  9,  pb_din, pb_doutset, pb_doutclr, mode9,  pb_modeh),
    PB10: (pb10, 10, pb_din, pb_doutset, pb_doutclr, mode10, pb_modeh),
    PB11: (pb11, 11, pb_din, pb_doutset, pb_doutclr, mode11, pb_modeh),
    PB12: (pb12, 12, pb_din, pb_doutset, pb_doutclr, mode12, pb_modeh),
    PB13: (pb13, 13, pb_din, pb_doutset, pb_doutclr, mode13, pb_modeh),
    PB14: (pb14, 14, pb_din, pb_doutset, pb_doutclr, mode14, pb_modeh),
    PB15: (pb15, 15, pb_din, pb_doutset, pb_doutclr, mode15, pb_modeh),
    PC0:  (pc0,  0,  pc_din, pc_doutset, pc_doutclr, mode0,  pc_model),
    PC1:  (pc1,  1,  pc_din, pc_doutset, pc_doutclr, mode1,  pc_model),
    PC2:  (pc2,  2,  pc_din, pc_doutset, pc_doutclr, mode2,  pc_model),
    PC3:  (pc3,  3,  pc_din, pc_doutset, pc_doutclr, mode3,  pc_model),
    PC4:  (pc4,  4,  pc_din, pc_doutset, pc_doutclr, mode4,  pc_model),
    PC5:  (pc5,  5,  pc_din, pc_doutset, pc_doutclr, mode5,  pc_model),
    PC6:  (pc6,  6,  pc_din, pc_doutset, pc_doutclr, mode6,  pc_model),
    PC7:  (pc7,  7,  pc_din, pc_doutset, pc_doutclr, mode7,  pc_model),
    PC8:  (pc8,  8,  pc_din, pc_doutset, pc_doutclr, mode8,  pc_modeh),
    PC9:  (pc9,  9,  pc_din, pc_doutset, pc_doutclr, mode9,  pc_modeh),
    PC10: (pc10, 10, pc_din, pc_doutset, pc_doutclr, mode10, pc_modeh),
    PC11: (pc11, 11, pc_din, pc_doutset, pc_doutclr, mode11, pc_modeh),
    PC12: (pc12, 12, pc_din, pc_doutset, pc_doutclr, mode12, pc_modeh),
    PC13: (pc13, 13, pc_din, pc_doutset, pc_doutclr, mode13, pc_modeh),
    PC14: (pc14, 14, pc_din, pc_doutset, pc_doutclr, mode14, pc_modeh),
    PC15: (pc15, 15, pc_din, pc_doutset, pc_doutclr, mode15, pc_modeh),
    PD0:  (pd0,  0,  pd_din, pd_doutset, pd_doutclr, mode0,  pd_model),
    PD1:  (pd1,  1,  pd_din, pd_doutset, pd_doutclr, mode1,  pd_model),
    PD2:  (pd2,  2,  pd_din, pd_doutset, pd_doutclr, mode2,  pd_model),
    PD3:  (pd3,  3,  pd_din, pd_doutset, pd_doutclr, mode3,  pd_model),
    PD4:  (pd4,  4,  pd_din, pd_doutset, pd_doutclr, mode4,  pd_model),
    PD5:  (pd5,  5,  pd_din, pd_doutset, pd_doutclr, mode5,  pd_model),
    PD6:  (pd6,  6,  pd_din, pd_doutset, pd_doutclr, mode6,  pd_model),
    PD7:  (pd7,  7,  pd_din, pd_doutset, pd_doutclr, mode7,  pd_model),
    PD8:  (pd8,  8,  pd_din, pd_doutset, pd_doutclr, mode8,  pd_modeh),
    PD9:  (pd9,  9,  pd_din, pd_doutset, pd_doutclr, mode9,  pd_modeh),
    PD10: (pd10, 10, pd_din, pd_doutset, pd_doutclr, mode10, pd_modeh),
    PD11: (pd11, 11, pd_din, pd_doutset, pd_doutclr, mode11, pd_modeh),
    PD12: (pd12, 12, pd_din, pd_doutset, pd_doutclr, mode12, pd_modeh),
    PD13: (pd13, 13, pd_din, pd_doutset, pd_doutclr, mode13, pd_modeh),
    PD14: (pd14, 14, pd_din, pd_doutset, pd_doutclr, mode14, pd_modeh),
    PD15: (pd15, 15, pd_din, pd_doutset, pd_doutclr, mode15, pd_modeh),
    PE0:  (pe0,  0,  pe_din, pe_doutset, pe_doutclr, mode0,  pe_model),
    PE1:  (pe1,  1,  pe_din, pe_doutset, pe_doutclr, mode1,  pe_model),
    PE2:  (pe2,  2,  pe_din, pe_doutset, pe_doutclr, mode2,  pe_model),
    PE3:  (pe3,  3,  pe_din, pe_doutset, pe_doutclr, mode3,  pe_model),
    PE4:  (pe4,  4,  pe_din, pe_doutset, pe_doutclr, mode4,  pe_model),
    PE5:  (pe5,  5,  pe_din, pe_doutset, pe_doutclr, mode5,  pe_model),
    PE6:  (pe6,  6,  pe_din, pe_doutset, pe_doutclr, mode6,  pe_model),
    PE7:  (pe7,  7,  pe_din, pe_doutset, pe_doutclr, mode7,  pe_model),
    PE8:  (pe8,  8,  pe_din, pe_doutset, pe_doutclr, mode8,  pe_modeh),
    PE9:  (pe9,  9,  pe_din, pe_doutset, pe_doutclr, mode9,  pe_modeh),
    PE10: (pe10, 10, pe_din, pe_doutset, pe_doutclr, mode10, pe_modeh),
    PE11: (pe11, 11, pe_din, pe_doutset, pe_doutclr, mode11, pe_modeh),
    PE12: (pe12, 12, pe_din, pe_doutset, pe_doutclr, mode12, pe_modeh),
    PE13: (pe13, 13, pe_din, pe_doutset, pe_doutclr, mode13, pe_modeh),
    PE14: (pe14, 14, pe_din, pe_doutset, pe_doutclr, mode14, pe_modeh),
    PE15: (pe15, 15, pe_din, pe_doutset, pe_doutclr, mode15, pe_modeh),
    PF0:  (pf0,  0,  pf_din, pf_doutset, pf_doutclr, mode0,  pf_model),
    PF1:  (pf1,  1,  pf_din, pf_doutset, pf_doutclr, mode1,  pf_model),
    PF2:  (pf2,  2,  pf_din, pf_doutset, pf_doutclr, mode2,  pf_model),
    PF3:  (pf3,  3,  pf_din, pf_doutset, pf_doutclr, mode3,  pf_model),
    PF4:  (pf4,  4,  pf_din, pf_doutset, pf_doutclr, mode4,  pf_model),
    PF5:  (pf5,  5,  pf_din, pf_doutset, pf_doutclr, mode5,  pf_model),
    PF6:  (pf6,  6,  pf_din, pf_doutset, pf_doutclr, mode6,  pf_model),
    PF7:  (pf7,  7,  pf_din, pf_doutset, pf_doutclr, mode7,  pf_model),
    PF8:  (pf8,  8,  pf_din, pf_doutset, pf_doutclr, mode8,  pf_modeh),
    PF9:  (pf9,  9,  pf_din, pf_doutset, pf_doutclr, mode9,  pf_modeh),
    PF10: (pf10, 10, pf_din, pf_doutset, pf_doutclr, mode10, pf_modeh),
    PF11: (pf11, 11, pf_din, pf_doutset, pf_doutclr, mode11, pf_modeh),
    PF12: (pf12, 12, pf_din, pf_doutset, pf_doutclr, mode12, pf_modeh),
    PF13: (pf13, 13, pf_din, pf_doutset, pf_doutclr, mode13, pf_modeh),
    PF14: (pf14, 14, pf_din, pf_doutset, pf_doutclr, mode14, pf_modeh),
    PF15: (pf15, 15, pf_din, pf_doutset, pf_doutclr, mode15, pf_modeh),
]);
