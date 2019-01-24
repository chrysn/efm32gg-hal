0.3.0
=====

* Add a timer module that can produce PWM output.

  This currently supports both the Pwm and the PwmPin interfaces, with
  different approaches to routing depending on whether the EFRxG1 or EFM32GG
  chips are used.

* Add I2C support.

* GPIO: Add support for open-drain pins, implement StatefulOutputPin

0.2.0
=====

* Incompatible changes in GPIO split and conversion methods, pin types moved
  into per-bank submodules

* Added very limited CMU abstraction that provides a systick-based delay
  implementation

* Added support for non-EFM32GG devices
