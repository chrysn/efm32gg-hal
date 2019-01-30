//! The full list was manually entered from the EFM32GG990 data sheet, which should be the most
//! comprehensive version. (The EFM32GG series was not observed to have any conflicts between its
//! members, so if anything is missing here that's present on another GG, it should be possible to
//! just add it.)

use crate::timer::{Timer0, Timer1, Timer2, Timer3, Channel0, Channel1, Channel2};
use super::per_peripheral::timerperipheral_pin;

// Routes with pins on duplicate routes still need some macro adjustment

timerperipheral_pin!(Timer0, Channel0, PA0, is_loc0, cc0pen);
// timerperipheral_pin!(Timer0, Channel0, PA0, is_loc1, cc0pen);
timerperipheral_pin!(Timer0, Channel0, PF6, is_loc2, cc0pen);
timerperipheral_pin!(Timer0, Channel0, PD1, is_loc3, cc0pen);
// timerperipheral_pin!(Timer0, Channel0, PA0, is_loc4, cc0pen);
timerperipheral_pin!(Timer0, Channel0, PF0, is_loc5, cc0pen);

timerperipheral_pin!(Timer0, Channel1, PA1, is_loc0, cc1pen);
// timerperipheral_pin!(Timer0, Channel1, PA1, is_loc1, cc1pen);
timerperipheral_pin!(Timer0, Channel1, PF7, is_loc2, cc1pen);
timerperipheral_pin!(Timer0, Channel1, PD2, is_loc3, cc1pen);
timerperipheral_pin!(Timer0, Channel1, PC0, is_loc4, cc1pen);
timerperipheral_pin!(Timer0, Channel1, PF1, is_loc5, cc1pen);

timerperipheral_pin!(Timer0, Channel2, PA2, is_loc0, cc2pen);
// timerperipheral_pin!(Timer0, Channel2, PA2, is_loc1, cc2pen);
timerperipheral_pin!(Timer0, Channel2, PF8, is_loc2, cc2pen);
timerperipheral_pin!(Timer0, Channel2, PD3, is_loc3, cc2pen);
timerperipheral_pin!(Timer0, Channel2, PC1, is_loc4, cc2pen);
timerperipheral_pin!(Timer0, Channel2, PF2, is_loc5, cc2pen);

timerperipheral_pin!(Timer1, Channel0, PE10, is_loc1, cc0pen);
timerperipheral_pin!(Timer1, Channel0, PB0, is_loc2, cc0pen);
timerperipheral_pin!(Timer1, Channel0, PB7, is_loc3, cc0pen);
timerperipheral_pin!(Timer1, Channel0, PD6, is_loc4, cc0pen);

timerperipheral_pin!(Timer1, Channel1, PE11, is_loc1, cc1pen);
timerperipheral_pin!(Timer1, Channel1, PB1, is_loc2, cc1pen);
timerperipheral_pin!(Timer1, Channel1, PB8, is_loc3, cc1pen);
timerperipheral_pin!(Timer1, Channel1, PD7, is_loc4, cc1pen);

timerperipheral_pin!(Timer1, Channel2, PE12, is_loc1, cc2pen);
timerperipheral_pin!(Timer1, Channel2, PB2, is_loc2, cc2pen);
timerperipheral_pin!(Timer1, Channel2, PB11, is_loc3, cc2pen);

timerperipheral_pin!(Timer2, Channel0, PA8, is_loc0, cc0pen);
timerperipheral_pin!(Timer2, Channel0, PA12, is_loc1, cc0pen);
timerperipheral_pin!(Timer2, Channel0, PC8, is_loc2, cc0pen);

timerperipheral_pin!(Timer2, Channel1, PA9, is_loc0, cc1pen);
timerperipheral_pin!(Timer2, Channel1, PA13, is_loc1, cc1pen);
timerperipheral_pin!(Timer2, Channel1, PC9, is_loc2, cc1pen);

timerperipheral_pin!(Timer2, Channel2, PA10, is_loc0, cc2pen);
timerperipheral_pin!(Timer2, Channel2, PA14, is_loc1, cc2pen);
timerperipheral_pin!(Timer2, Channel2, PC10, is_loc2, cc2pen);

timerperipheral_pin!(Timer3, Channel0, PE14, is_loc0, cc0pen);
timerperipheral_pin!(Timer3, Channel0, PE0, is_loc1, cc0pen);

timerperipheral_pin!(Timer3, Channel1, PE15, is_loc0, cc1pen);
timerperipheral_pin!(Timer3, Channel1, PE1, is_loc1, cc1pen);

timerperipheral_pin!(Timer3, Channel2, PA15, is_loc0, cc2pen);
timerperipheral_pin!(Timer3, Channel2, PE2, is_loc1, cc2pen);
