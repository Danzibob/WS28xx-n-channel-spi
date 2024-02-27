//! Raspberry Pi (Model 1) SPI adaptor implementation.
//!
//! If you can't use the optional "adapter_spidev"-feature then take this as a template and build your own encoding functions.
//! So far this file and it's functions are static and not dynamic.
//!
//! The WS28XX has specific restrictions on how long high and low signal must be send on DIN-wire in nanoseconds.
//! All logic and constants needed to cope with this are placed here.
//!
//! Please note that we have to cope with high frequencies which can be really tricky.
//! Perhaps you need other timings on your device.
//! This was tested on a Raspberry Pi with its SPI device.
//!
//! See device specification for further details.

use crate::std::io;
use crate::std::io::Write;
use crate::GenericHardware;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};

/// The frequency for the SPI device on the Raspberry Pi (Model 1).
///
/// Works on other Linux systems with SPI device probably too if they have a similar frequency.
pub const PI_SPI_HZ: u32 = 15_600_000;
// see https://web.archive.org/web/20140808235913/https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md

// this means 1 / 15_600_000 * 1E9 ns/cycle => 64ns / cycle => 15.6 MBit/s
//
// See data sheet: https://cdn-shop.adafruit.com/datasheets/WS2812.pdf
//
// Timings of WS2818:
//
// pub const _T0H_NS: u64 = 350; // ±150ns tolerance
// pub const _T0L_NS: u64 = 800; // ±150ns tolerance
// pub const _T1H_NS: u64 = 700; // ±150ns tolerance
// pub const _T1L_NS: u64 = 600; // ±150ns tolerance
// pub const _TRESET: u64 = 50_000; // >50 µs
//
// One Wire Protocol on WS2812 requires the
// - "logical 0 Bit" to be:
//   - T0H_NS ±150ns to be high
//   - T0L_NS ±150ns to be low     (most of the time; at the end)
// - "logical 1 Bit" to be:
//   - T1H_NS ±150ns to be high    (most of the time; at the beginning)
//   - T1L_NS ±150ns to be low
//
// T0H_NS = 350ns ± 150ns => 1_1111          ( 5 bits * 64ns per bit ~ 320ns)
// T0L_NS = 800ns ± 150ns => 000_0000_0000   (11 bits * 64ns per bit ~ 704ns)
//
// T1H_NS = 700ns ± 150ns => 1_1111_1111    (9 bits * 64ns per bit ~ 576ns)
// T1L_NS = 600ns ± 150ns => 000_0000        (7 bits * 64ns per bit ~ 448ns)
//
// => !! we encode one data bit in two SPI byte for the proper timings !!

/// How many SPI bytes must be sent for a single data bit.
///
/// This number of bytes result in one logical zero or one
/// on WS28xx LED.
pub const SPI_BYTES_PER_BIT: usize = 2;

/// Number of SPI bytes for the 50us reset signal
pub const SPI_BYTES_PER_RESET: usize = 100;

/// See code comments above where this value comes from!
///
/// These are the bits to send via SPI MOSI that represent a logical 0 on WS28xx RGB LED interface.
/// Frequency + length results in the proper timings.
pub const WS_ZERO_BYTES: [u8; SPI_BYTES_PER_BIT] = [0b1111_1000, 0b0000_0000];

/// See code comments above where this value comes from!
///
/// These are the bits to send via SPI MOSI that represent a logical 1 on WS28xx RGB LED interface.
/// Frequency + length results in the proper timings.
pub const WS_ONE_BYTES: [u8; SPI_BYTES_PER_BIT] = [0b1111_1111, 0b1000_0000];

/// Number of bits per pixel; should not be changed.
pub const BITS_PER_PX: usize = 8; // should always be 8, but left as a constant for easy editing
const SPI_BYTES_PER_PX: usize = SPI_BYTES_PER_BIT * BITS_PER_PX;

/// Colour information for a single arbitrary size node (e.g. one RGB unit)
pub type Node<const N: usize> = [u8; N];

/// Encodes a pixel value to the bytes that must be transferred via SPI MOSI.
///
/// These SPI bytes represent the logical zeros and ones for WS2818.
/// The resulting is [`SPI_BYTES_PER_PX`] bytes long.
pub fn encode_pixel(pixel: &u8) -> [u8; SPI_BYTES_PER_PX] {
    // Initialize empty array for SPI bytes for this pixel
    let mut encoded: [u8; SPI_BYTES_PER_PX] = [0; SPI_BYTES_PER_PX];
    // Iterate through each bit of the pixel value specified
    for px_bit_idx in 0..(BITS_PER_PX) {
        let bit = (pixel >> (BITS_PER_PX - px_bit_idx - 1)) & 1;
        // Select the correct SPI bytes and set them in the output
        let spi_data = if bit == 1 {
            WS_ONE_BYTES
        } else {
            WS_ZERO_BYTES
        };
        for byte in 0..SPI_BYTES_PER_BIT {
            encoded[px_bit_idx * 2 + byte] = spi_data[byte];
        }
    }
    encoded
}

/// Encodes one arbitrary sized Node of LEDs.
///
/// Uses [`encode_pixel`] for each value.
pub fn encode_node<const N: usize>(node: &Node<N>) -> [u8; N * SPI_BYTES_PER_PX] {
    let mut output: [u8; N * SPI_BYTES_PER_PX] = [0; N * SPI_BYTES_PER_PX];
    for (i, px) in node.iter().enumerate() {
        let bytes = encode_pixel(px);
        for (j, byte) in bytes.iter().enumerate() {
            output[i * SPI_BYTES_PER_PX + j] = *byte;
        }
    }
    output
}

/// Wrapper around Spidev.
pub struct PiSPI<const B: usize>
where
    [u8; SPI_BYTES_PER_RESET + B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
{
    spi: Spidev,
    buffer: [u8; SPI_BYTES_PER_RESET + B * SPI_BYTES_PER_BIT * BITS_PER_PX],
}

// Implement Hardware abstraction for device.
impl<const B: usize> GenericHardware<B> for PiSPI<B>
where
    [u8; SPI_BYTES_PER_RESET + B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
{
    type Error = std::io::Error;

    fn write_raw(&mut self, encoded_data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write_all(encoded_data)
    }

    fn encode_and_write(&mut self, node_data: &[u8]) -> Result<(), Self::Error> {
        for (i, byte) in node_data.iter().flat_map(encode_pixel).enumerate() {
            self.buffer[i + SPI_BYTES_PER_RESET] = byte;
        }
        self.spi.write_all(&self.buffer)
    }
}

impl<const B: usize> PiSPI<B>
where
    [u8; SPI_BYTES_PER_RESET + B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
{
    /// Connects your application with the SPI-device of your device.
    ///
    /// This uses the `spidev`-crate.
    /// Returns a new adapter object for the WS28xx LEDs.
    ///
    /// * `dev` - Device name. Probably "/dev/spidev0.0" if available.
    ///
    /// Fails if connection to SPI can't be established.
    pub fn new(dev: &str) -> io::Result<Self> {
        let mut spi = Spidev::open(dev)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(PI_SPI_HZ)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        Ok(Self {
            spi,
            buffer: [0; SPI_BYTES_PER_RESET + B * SPI_BYTES_PER_BIT * BITS_PER_PX],
        })
    }
}
