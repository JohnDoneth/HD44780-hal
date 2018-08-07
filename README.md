# hd44780-hal

[![crates.io](https://img.shields.io/crates/v/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)
[![crates.io](https://img.shields.io/crates/d/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)
[![crates.io](https://img.shields.io/crates/l/hd44780-hal.svg)](https://crates.io/crates/hd44780-hal)

Implementation of the `embedded-hal` traits for the HD44780.

![](/header.gif)


### Documentation

https://docs.rs/hd44780-hal

### Example usage

This library is pretty bare bones at the moment, but it's greatest feature is that all you need to do to use it is supply the `HD44780::new` function a bunch of pins that implement the `OutputPin` trait for [embedded-hal](https://github.com/rust-embedded/embedded-hal) as well as a struct that implements delay with support for `DelayUs<u16> + DelayMs<u8>` also from [embedded-hal](https://github.com/rust-embedded/embedded-hal).

```rust
// Code grabbed from the metro_m0 example
let mut lcd = HD44780::new(
    
    pins.d4.into_open_drain_output(&mut pins.port), // Register Select pin
    pins.d3.into_open_drain_output(&mut pins.port), // Enable pin

    pins.d5.into_open_drain_output(&mut pins.port),  // d0
    pins.d6.into_open_drain_output(&mut pins.port),  // d1
    pins.d7.into_open_drain_output(&mut pins.port),  // d2
    pins.d8.into_open_drain_output(&mut pins.port),  // d3
    pins.d9.into_open_drain_output(&mut pins.port),  // d4
    pins.d10.into_open_drain_output(&mut pins.port), // d5
    pins.d11.into_open_drain_output(&mut pins.port), // d6
    pins.d12.into_open_drain_output(&mut pins.port), // d7

    delay,
);

lcd.reset();
lcd.clear();
lcd.set_display_mode(true, true, true);
lcd.write_str("Hello, world!");
```

## Todo's

- 4 bit mode
- Busy flag support
- Non-blocking API
- Make the API for user-friendly
- Raspberry Pi example using [linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal)

Additional issues as well as pull-requests are welcome!

## License

This project is licensed under MIT license ([LICENSE](https://github.com/kunerd/clerk/blob/master/docs/CONTRIBUTING.md) or <https://opensource.org/licenses/MIT>)