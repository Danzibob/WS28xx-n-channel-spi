//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws28xx_n_channel_spi::adapter_gen::WS28xxAdapter;
use ws28xx_n_channel_spi::adapter_spi::WS28xxSpiAdapter;
use ws28xx_n_channel_spi::encoding::encode_slice;

use std::time::{Duration, Instant};

// Example that shows a single moving pixel though the 8x8 led matrix.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = get_led_num_from_args();

    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossiframe_delayble to reach

    let mut i = 0;
    loop {
        let mut data = vec![];
        for j in 0..num_leds {
            // fill num_leds-1 pixels with black; one with white
            if i == j {
                data.extend_from_slice(&encode_slice(&[50, 50, 50]));
            } else {
                data.extend_from_slice(&encode_slice(&[0, 0, 0]));
            }
        }
        adapter.write_encoded_slice(&data).unwrap();

        i = (i + 1) % num_leds;
        sleep_busy_waiting_ms(100);
    }
}


/// Returns n from args or default.
pub fn get_led_num_from_args() -> usize {
    println!(
        "You can provide the number of LEDs as argument when calling from command line.\
        For example \"cargo run --bin <bin> 64\". The default is 64."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let leds = args[1].parse::<usize>();
        if let Result::Ok(leds) = leds {
            println!("Using {} LEDs", leds);
            return leds;
        }
    }

    // Default
    println!("Using 64 LEDs");
    64
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