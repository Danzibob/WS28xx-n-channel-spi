#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::time::{Duration, Instant};
use ws28xx_n_channel_spi::pi_spi::PiSPI;
use ws28xx_n_channel_spi::LEDs;

// 3 channels per module is a standard RGB setup
const CHANNELS_PER_MODULE: usize = 3;
// Number of modules
const NUM_MODULES: usize = 64;
// Using 64 LEDs for an 8x8 grid as a demonstration
const NUM_LEDS: usize = NUM_MODULES * CHANNELS_PER_MODULE;

// Example that shows a single moving pixel though an RGB 8x8 led matrix.
fn main(){
    // Create the linux SPI device adapter
    let hw_adapter: PiSPI<NUM_LEDS> = PiSPI::new("/dev/spidev1.0").unwrap();
    // Create an LED strip with
    let mut strip: LEDs<NUM_LEDS, CHANNELS_PER_MODULE, PiSPI<NUM_LEDS>> = LEDs::new(hw_adapter);

    // Colour order is GRB for standard NeoPixels
    const PURPLE: [u8; 3] = [0, 50, 30];
    const OFF: [u8; 3] = [0, 0, 0];

    let mut i: usize = 0;
    loop {

        // Try to run at 60fps
        let next_frame = Instant::now() + Duration::from_millis(16);

        strip.set_node(i, OFF);

        i = (i + 1) % (NUM_MODULES);

        strip.set_node(i, PURPLE);

        strip.write().unwrap();

        while Instant::now() < next_frame {}
    }
}
