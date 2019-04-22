use super::per_function::timerchannel_pin;
use crate::timer::{
    Channel0, Channel1, Channel2, Channel3, Timer0, Timer1, Timer2, Timer3, Timer4, Timer5, Timer6,
};

timerchannel_pin!(Timer0, Channel0, PA0, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PF6, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PD1, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PB6, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PF0, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PC4, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PA8, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer0, Channel0, PA1, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer0, Channel1, PA1, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PF7, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PD2, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PC0, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PF1, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PC5, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PA9, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer0, Channel1, PA0, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer0, Channel2, PA2, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PF8, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PD3, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PC1, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PF2, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PA7, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PA10, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer0, Channel2, PA13, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer1, Channel0, PC13, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PE10, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PB0, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PB7, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PD6, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PF2, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PF13, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer1, Channel0, PI6, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer1, Channel1, PC14, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PE11, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PB1, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PB8, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PD7, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PF3, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PF14, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer1, Channel1, PI7, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer1, Channel2, PC15, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PE12, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PB2, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PB11, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PC13, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PF4, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PF15, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer1, Channel2, PI8, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer1, Channel3, PC12, loc0, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PE13, loc1, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PB3, loc2, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PB12, loc3, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PC14, loc4, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PF12, loc5, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PF5, loc6, cc3loc, cc3pen);
timerchannel_pin!(Timer1, Channel3, PI9, loc7, cc3loc, cc3pen);

timerchannel_pin!(Timer2, Channel0, PA8, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PA12, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PC8, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PF2, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PB6, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PC2, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PG8, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer2, Channel0, PG5, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer2, Channel1, PA9, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PA13, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PC9, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PE12, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PC0, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PC3, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PG9, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer2, Channel1, PG6, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer2, Channel2, PA10, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PA14, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PC10, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PE13, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PC1, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PC4, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PG10, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer2, Channel2, PG7, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer3, Channel0, PE14, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PE0, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PE3, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PE5, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PA0, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PA3, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PA6, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer3, Channel0, PD15, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer3, Channel1, PE15, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PE1, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PE4, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PE6, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PA1, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PA4, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PD13, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer3, Channel1, PB15, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer3, Channel2, PA15, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PE2, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PE5, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PE7, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PA2, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PA5, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PD14, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer3, Channel2, PB0, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer4, Channel0, PF3, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PF13, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PF5, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PF6, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PI8, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PF9, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PD11, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer4, Channel0, PE9, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer4, Channel1, PF4, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PF14, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PI6, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PI9, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PF7, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PD9, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PD12, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer4, Channel1, PE10, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer4, Channel2, PF12, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PF15, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PI7, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PI10, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PF8, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PD10, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PE8, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer4, Channel2, PE11, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer5, Channel0, PE4, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PE7, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PH13, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PI0, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PC8, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PC11, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PC14, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer5, Channel0, PF12, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer5, Channel1, PE5, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PH11, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PH14, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PI1, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PC9, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PC12, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PF10, loc6, cc1loc, cc1pen);
timerchannel_pin!(Timer5, Channel1, PF13, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer5, Channel2, PE6, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PH12, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PH15, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PI2, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PC10, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PC13, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PF11, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer5, Channel2, PF14, loc7, cc2loc, cc2pen);

timerchannel_pin!(Timer6, Channel0, PG0, loc0, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PG6, loc1, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PG12, loc2, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PH2, loc3, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PH8, loc4, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PB13, loc5, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PD1, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel0, PD4, loc7, cc0loc, cc0pen);

timerchannel_pin!(Timer6, Channel1, PG1, loc0, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PG7, loc1, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PG13, loc2, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PH3, loc3, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PH9, loc4, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PB14, loc5, cc1loc, cc1pen);
timerchannel_pin!(Timer6, Channel1, PD2, loc6, cc0loc, cc0pen);
timerchannel_pin!(Timer6, Channel1, PD5, loc7, cc1loc, cc1pen);

timerchannel_pin!(Timer6, Channel2, PG2, loc0, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PG8, loc1, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PG14, loc2, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PH4, loc3, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PH10, loc4, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PD0, loc5, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PD3, loc6, cc2loc, cc2pen);
timerchannel_pin!(Timer6, Channel2, PD6, loc7, cc2loc, cc2pen);
