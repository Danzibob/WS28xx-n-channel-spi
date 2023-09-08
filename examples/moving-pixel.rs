//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details
use ws28xx_n_channel_spi::generic_adapter::*;
use ws28xx_n_channel_spi::linux_spi::LinuxSPI;
use std::time::{Duration, Instant};

// 3 channels per module is a standard RGB setup
const CHANNELS_PER_MODULE : usize = 3;
// Number of modules
const NUM_MODULES : usize = 64;
// Using 64 LEDs for an 8x8 grid as a demonstration
const NUM_LEDS : usize = NUM_MODULES * CHANNELS_PER_MODULE;

// Example that shows a single moving pixel though an RGB 8x8 led matrix.
fn main() {
    // Create the linux SPI device adapter
    let hw_adapter : LinuxSPI<NUM_LEDS> = LinuxSPI::new("/dev/spidev0.0").unwrap();
    // Create an LED strip with 
    let mut strip : LEDs<NUM_LEDS, CHANNELS_PER_MODULE, LinuxSPI<NUM_LEDS>> = LEDs::new(hw_adapter);

    // Colour order is GRB for standard NeoPixels
    const PURPLE: [u8;3] = [0, 50, 30];
    const OFF: [u8;3] = [0, 0, 0];

    let mut i: usize = 0;
    loop {

        strip.set_node(i, OFF);

        i = (i + 1) % (NUM_MODULES);

        strip.set_node(i, PURPLE);

        let _ = strip.write();

        sleep_busy_waiting_ms(100);
    }
}

#[inline(always)]
pub fn sleep_busy_waiting_ms(ms: u64) {
    let target_time = Instant::now() + Duration::from_millis(ms);
    loop {
        if Instant::now() >= target_time {
            break;
        }
    }
}