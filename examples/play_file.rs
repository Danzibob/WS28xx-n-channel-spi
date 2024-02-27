#![feature(generic_const_exprs)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use ws28xx_n_channel_spi::pi_spi::PiSPI;
use ws28xx_n_channel_spi::LEDs;

// 3 channels per module is a standard RGB setup
const CHANNELS_PER_MODULE: usize = 9;
// Number of modules
const NUM_MODULES: usize = 64;
// Using 64 LEDs for an 8x8 grid as a demonstration
const NUM_LEDS: usize = NUM_MODULES * CHANNELS_PER_MODULE;

#[derive(Debug)]
enum Command<const N: usize> {
    Show,
    Clear,
    Sleep { ms: u64, us: u64},
    Data { id: usize, leds: [u8; N] },
}


fn read_lines<P>(filename: P) -> io::Result<Vec<Command<CHANNELS_PER_MODULE>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|l| {
        let line = l.ok()?;
        if line.starts_with("#") {return None}
        match line.split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {

            // Show (no arguments)
            ["show", ..] => { Some(Command::Show) },

            // Show (no arguments)
            ["clear", ..] => { Some(Command::Clear) },

            // Sleep [time_ms] <time_us>
            ["sleep", time_ms, time_us @ ..] => {
                let ms = time_ms.parse::<u64>()
                        .expect("Parse error for time_ms field");
                let us = time_us.first().map(|x| x.parse::<u64>()
                        .expect("Parse error for time_us field"))
                        .unwrap_or(0);
                Some(Command::Sleep{ms, us})
            },

            // Data line
            [id_str, ref values @ ..] => {
                if values.len() != CHANNELS_PER_MODULE { panic!("Wrong length data line") }

                // TODO: Potentially add check for configured num modules?
                let id = id_str.parse::<usize>().expect("Invalid node ID");

                let mut items = values.iter();
                let mut leds: [u8; CHANNELS_PER_MODULE] = [0; CHANNELS_PER_MODULE];
                for i in 0..CHANNELS_PER_MODULE {
                    if let Some(val) = items.next(){
                        leds[i] = val.parse::<u8>().expect("Invalid data item")
                    } else {panic!("Invalid value in data line")}
                }
                Some( Command::Data { id, leds })
            },
            _ => None
        }
    }).collect())
}

fn main() {
    // Identify and load file
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("No filename provided");

    let commands = read_lines(filepath).expect("Failed to open file");

    // Create the linux SPI device adapter
    let hw_adapter: PiSPI<NUM_LEDS> = PiSPI::new("/dev/spidev1.0")
                                    .expect("Failed to open SPI device");
    // Create an LED strip
    let mut strip: LEDs<NUM_LEDS, CHANNELS_PER_MODULE, PiSPI<NUM_LEDS>> = LEDs::new(hw_adapter);

    for command in commands {
        match command {
            Command::Data { id, leds } => strip.set_node(id, leds),
            Command::Show  => strip.write()
                             .expect("Failed to write to strip"),
            Command::Clear => strip.clear()
                             .expect("Failed to clear strip"),
            Command::Sleep { ms, us } => {
                let next_frame = Instant::now() + Duration::from_micros(ms * 1000 + us);
                while Instant::now() < next_frame {}
            }
        }
        println!("{:?}", command);
    }
}

/*
File format:

- Commented lines start with a #
- Commands are: 
    - "show" (Pushes current buffer)
    - "sleep [time_ms] <time_ns>" (waits given amount of time)
    - "clear" (Turns off all LEDs)
- No command implies this is a data line: node_id [led_1] [led_2] ... [led_n]

Example

#id R   G   B
0   255 0   0
1   0   255 0
2   0   0   255
show
# Display LEDs for 1 second
sleep 1000
clear
*/