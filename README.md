`efm32gg-hal`
-------------

> **Note**: Renaming of this crate to `em32-hal` is pending, along with the
> move of the crate into the [em32-wg] group.

This is an implementation of the [embedded-hal] API for various Silicon Labs
devices in the lineage around the [EFM32 Giant Gecko].

Currently, the EFM32GG (Giant Gecko) and EFR32xG1 (original configuration of
Mighty, Blue and Flex Gecko) are supported. The complexity of adding chip
families probably varies from just adding the device as "works like that other
device" (should apply to most EFM32 devices released before around 2016) to
"Some functionality is not supported any more and needs workarounds" (eg. when
bit-band access becomes mandatory for atomic operations on a register). On the
long run, it can be expected to cover the devices supported by the vendor's
[emlib] C library.

The actually used chip is selected by features named like "chip-efm32gg" or
"chip-efr32x1"; pick an extern crate to load as "registers" and add the
appropriate dependency to a crate typically created by [svd2rust].

[em32-wg]: https://github.com/em32-rs/wg
[embedded-hal]: https://github.com/japaric/embedded-hal
[EFM32 Giant Gecko]: https://www.silabs.com/products/mcu/32-bit/efm32-giant-gecko
[svd2rust]: https://github.com/japaric/svd2rust
[efm32gg990]: https://crates.io/crates/efm32gg990
[peripheral description document]: https://www.silabs.com/documents/public/reference-manuals/EFM32GG-RM.pdf
[emlib]: http://devtools.silabs.com/dl/documentation/doxygen/

### State of implementation

This is very immature software. So far, only GPIO pins are described in terms
of the HAL's ``digital`` interface.

### Usage

The board support crates for [EFM32GG-STK3700] and [Thunderboard (SLTB001a)]
contain examples of how this crate can be used.

[EFM32GG-STK3700]: https://github.com/chrysn/efm32gg-stk3700
[Thunderboard (SLTB001a)]: https://github.com/chrysn/thunderboard-sltb001a

### License

This is licensed under the [Apache License] or the [MIT License] at the your
option. By contributing to this project, you license your contribution under
the same dual-licensed terms unless the contribution itself says otherwise.

[Apache License]: http://www.apache.org/licenses/LICENSE-2.0
[MIT License]: http://opensource.org/licenses/MIT
