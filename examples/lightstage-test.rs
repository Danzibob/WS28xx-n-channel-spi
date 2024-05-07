#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

// use ws28xx_n_channel_spi::pi_spi::PiSPI;
use std::time::{Duration, Instant};
use ws28xx_n_channel_spi::rpi_ws281x;
use ws28xx_n_channel_spi::LEDs;

// Number of nodes on the lightstage
const NUM_MODULES: usize = 143;
// Lightstage has 9 LED channels per node
const CHANNELS_PER_MODULE: usize = 9;

const NUM_LEDS: usize = NUM_MODULES * CHANNELS_PER_MODULE;

// Example to test every node on the UoY Lightstage
fn main() {
    // Create the linux SPI device adapter
    // let hw_adapter : PiSPI<NUM_LEDS> = PiSPI::new("/dev/spidev1.0").unwrap();
    let hw_adapter = rpi_ws281x::setup::<CHANNELS_PER_MODULE>(NUM_MODULES, 50).unwrap();
    // Create an LED strip with
    let mut strip: LEDs<NUM_LEDS, CHANNELS_PER_MODULE, _> = LEDs::new(hw_adapter);

    const PURPLE: [u8; 9] = [240, 60, 120, 200, 200, 200, 200, 200, 200];
    const OFF: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut i: usize = 0;
    loop {
        // Try ro run at 10fps
        let next_frame = Instant::now() + Duration::from_millis(100);

        strip.set_node(i, OFF);

        i = (i + 1) % (NUM_MODULES);

        strip.set_node(i, PURPLE);

        strip.write().unwrap();

        while Instant::now() < next_frame {}
    }
}
