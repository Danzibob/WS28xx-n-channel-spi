#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ws28xx_n_channel_spi::pi_spi::PiSPI;
use ws28xx_n_channel_spi::LEDs;

// 3 channels per module is a standard RGB setup
const CHANNELS_PER_MODULE : usize = 3;
// Number of modules
const NUM_MODULES : usize = 64;
// Using 64 LEDs for an 8x8 grid as a demonstration
const NUM_LEDS : usize = NUM_MODULES * CHANNELS_PER_MODULE;

// Example that shows a single moving pixel though an RGB 8x8 led matrix.
fn main(){
    // Create the linux SPI device adapter
    let hw_adapter : PiSPI<NUM_LEDS> = PiSPI::new("/dev/spidev1.0").unwrap();
    // Create an LED strip
    let mut strip : LEDs<NUM_LEDS, CHANNELS_PER_MODULE, _> = LEDs::new(hw_adapter);

    // Colour order is GRB for standard NeoPixels
    const PURPLE: [u8;3] = [0, 50, 30];

    for i in 0..NUM_MODULES {
        strip.set_node(i, PURPLE);
    }

    strip.write().unwrap();
}
