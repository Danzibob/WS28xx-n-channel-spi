# WS28xx RGB LED SPI Driver

This crate is a driver for WS28XX (WS2811, WS2812, WS2812B, WS2818) RGB LED chains/strips.
 
### About this driver

This driver was developed for a specific project where each LED module along a chain had 9 single-colour LED modules, and no available neopixel-esque driver seemed up to the task. Therefore this library can support arbitrary module sizes and doesn't reference RGB.

This driver only works on Linux systems with a SPI device, like Raspberry Pi [1]. This is needed because my driver operates at 15.6MHz. This is required because I need to reach specific *timings in nanoseconds* according to the specification while sending data [2].
WS28xx LEDs use a one wire protocol without a clock, therefore the timings during data transmission are important.

The SPI device in your Raspberry Pi has a reliable clock with high frequencies available. Regular GPIO pins **won't work!** Toggling GPIO pins takes 1µs (in my testing) which is *WAY TOO SLOW!* Therefore I use SPI. There is a clock device in hardware - much more reliable!

Find the `MOSI`-Pin on your device (e.g. Raspberry Pi) and connect it with `DIN`-Port of the LED. That's all what's needed.

**There is no warranty that this will work on your setup! High frequency stuff is complicated!**

![demo](ws2818-rgb-demo.gif) 

*Demo using a 8x8 RGB LED matrix. DIN is connected with MOSI (SPI Out Port).*

### Examples
See https://github.com/phip1611/ws2818-rgb-led-spi-driver/tree/master/examples. 

#### Cargo.toml
```toml
[dependencies]
ws2818-rgb-led-spi-driver = "<latest version>"
# or if you need no_std
ws2818-rgb-led-spi-driver = { version = "<latest version>", default-features = false }
```

##### Links

[1] https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md \
[2] https://cdn-shop.adafruit.com/datasheets/WS2812.pdf 
