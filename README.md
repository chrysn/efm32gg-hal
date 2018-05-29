`efm32gg-hal`
-------------

This is an implementation of the [embedded-hal] API for Silicon Labs [EFM32
Giant Gecko] devices.

It is based on the [svd2rust] generated [efm32gg990] crate, but is usable for
all EFM32GG chips. (All Gecko chips share a common [peripheral description
document] that is the main information source of this implementation; their
data sheets give individual numbers and addresses of peripherals, but it
appears they are all a strict superset of the largest Giant Gecko chips).

It might even be usable for other devices of the Gecko series (they have
separate peripheral documentation, but it appears that many peripherals are
just copied over); probably, this library will be developed into a generic
`efm32-hal` library that implements all of the variation there is.

[embedded-hal]: https://github.com/japaric/embedded-hal
[EFM32 Giant Gecko]: https://www.silabs.com/products/mcu/32-bit/efm32-giant-gecko
[svd2rust]: https://github.com/japaric/svd2rust
[ef32gg990]: https://crates.io/crates/efm32gg990
[peripheral description document]: https://www.silabs.com/documents/public/reference-manuals/EFM32GG-RM.pdf

### License

This is licensed under the [Apache License] or the [MIT License] at the your
option. By contributing to this project, you license your contribution under
the same dual-licensed terms unless the contribution itself says otherwise.

[Apache License]: http://www.apache.org/licenses/LICENSE-2.0
[MIT License]: http://opensource.org/licenses/MIT
