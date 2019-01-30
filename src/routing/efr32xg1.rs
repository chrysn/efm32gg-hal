//! Route information taken from [Mighty Gecko data
//! sheet](https://www.silabs.com/documents/public/data-sheets/efr32mg1-datasheet.pdf); Flex- and
//! Blue Gecko appear to be the same.

use crate::timer::{Timer0, Channel0, Channel1, Channel2};
use super::per_function::timerchannel_pin;

timerchannel_pin!(Timer0, Channel0, PA0, loc0, cc0loc, cc0pen);

timerchannel_pin!(Timer0, Channel0, PD11, loc19, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel1, PD12, loc19, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel2, PD13, loc19, cc2loc, cc2pen);
