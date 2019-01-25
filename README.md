# Smart-leds

Use various addresable led like the ws2812 (neopixel) or apa102 (dotstar) in
rust. As their protocol greatly differs and for some (like the ws2812) there
are various methods, this crate doesn't implement any specific drivers by
itself. It has to be coupled with a device driver that implements the
[smart-leds-trait](https://github.com/smart-leds-rs/smart-leds-trait), like the
[ws2812-spi](https://github.com/smart-leds-rs/ws2812-spi-rs), the
[ws2812-nop-samd21](https://github.com/smart-leds-rs/ws2812-nop-samd21) or the
[apa102-spi](https://github.com/smart-leds-rs/apa102-spi-rs) drivers.

You can see some usage examples in the
[smart-leds-samples](https://github.com/smart-leds-rs/smart-leds-samples) repo.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
