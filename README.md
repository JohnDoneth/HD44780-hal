# hd44780-hal

Implementation of the `embedded-hal` traits for the HD44780

![](/header.gif)



## Example usage

This library is pretty bare bones at the moment, but it's greatest feature is that all you need to do to use it is supply the HD44780 function a bunch of pins that implement the ```OutputPin``` trait for embedded-hal as well as a struct that implements delay with support for ```DelayUs<u16> + DelayMs<u8>```.

```rust
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



Issues and pull-requests are welcome!

Todo

- 4 bit mode
- Busy flag support
- Make the API for user-friendly
- Raspberry Pi example using [linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal)