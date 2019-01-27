/// Route information taken from [Mighty Gecko data
/// sheet](https://www.silabs.com/documents/public/data-sheets/efr32mg1-datasheet.pdf); Flex- and
/// Blue Gecko appear to be the same.

use crate::timer::{Timer0, Channel0, Channel1, Channel2};
use super::per_function::timerchannel_pin;

timerchannel_pin!(Timer0, Channel0, cc0loc, PA0, loc0);

timerchannel_pin!(Timer0, Channel0, cc0loc, PD11, loc19);
timerchannel_pin!(Timer0, Channel1, cc1loc, PD12, loc19);
timerchannel_pin!(Timer0, Channel2, cc2loc, PD13, loc19);
