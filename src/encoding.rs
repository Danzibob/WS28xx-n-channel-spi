//! Utility functions for the encoding of data. This functions respect the restrictions from `timings`-module.
//! This whole module works with and only with a frequency of [`crate::timings::PI_SPI_HZ`]. If you can't
//! use the optional "adapter_spidev"-feature then take this as a template and build your own encoding functions.
//! So far this file and it's functions are static and not dynamic.
use crate::timings::encoding::{
    SPI_BYTES_PER_BIT, WS_ONE_BYTES, WS_ZERO_BYTES
};
use alloc::vec::Vec;
const BITS_PER_PX: usize = 8; // only 0-8 will work as u8 is used in code
const SPI_BYTES_PER_PX: usize = SPI_BYTES_PER_BIT * BITS_PER_PX;

/// Encodes a pixel value to the bytes that must be transferred via SPI MOSI.
/// These SPI bytes represent the logical zeros and ones for WS2818.
/// This counts in the constraints that come from [`crate::timings`]-module.
/// The resulting is [`SPI_BYTES_PER_PX`] bytes long.
pub fn encode_pixel(pixel: &u8) -> [u8; SPI_BYTES_PER_PX]{
    // Initialize empty array for SPI bytes for this pixel
    let mut encoded: [u8; SPI_BYTES_PER_PX] = [0; SPI_BYTES_PER_PX];
    // Iterate through each bit of the pixel value specified
    for px_bit_idx in 0..(BITS_PER_PX){
        let bit = (pixel >> (BITS_PER_PX - px_bit_idx - 1)) & 1;
        // Select the correct SPI bytes and set them in the output
        let spi_data = if bit == 1 {WS_ONE_BYTES} else {WS_ZERO_BYTES};
        for byte in 0..SPI_BYTES_PER_BIT{
            encoded[px_bit_idx*2 + byte] = spi_data[byte];
        }
    }
    encoded
}

/// Encodes multiple LED values in a slice. Uses [`encode_pixel`] for each value.
pub fn encode_slice(pixels: &[u8]) -> Vec<u8>{
    pixels.iter().flat_map(encode_pixel).collect()
}
