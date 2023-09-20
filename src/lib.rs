//! This crate is intended to fill a nice of having an arbitraty number of colour channels
//! per node in a WS28xx setup. Previous crates focus on RGB and RGBW specifically, but
//! this crate allows for an arbitrary number of channels using generics.
//!
//! Through the use of a [`generic_adapter::GenericHardware`] trait, different methods
//! of driving the LEDs may be implemented. This library comes with an SPI bit-banging
//! implementation for the raspberry pi, but because the main library is `no-std` compatible,
//! alternate hardware implementations should be possible across any platform.

#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "adapter_spidev")]
extern crate std;
#[cfg(feature = "adapter_spidev")]
pub mod linux_spi; // specific [std]-implementation

// Raspberry Pi SPI device
// you can easily provide your own encoding functions.
pub mod linux_spi_encoding;

/// Hardware device abstraction, which can be implemented by many different types of back-end (embedded, linux, etc.)
pub trait GenericHardware<const B: usize> {
    type Error;

    /// Sequentially write `byte_array` exactly as presented.
    fn write_raw(&mut self, byte_array: &[u8]) -> Result<(), Self::Error>;

    /// Encode `byte_array` suitable for WS81XX bitbanging, then write it.
    fn encode_and_write(&mut self, byte_array: &[u8]) -> Result<(), Self::Error>;
}

/// Struct that contains a buffer of indiviual LED values (bytes)
///
/// The buffer is passed to the hardware device to be encoded
/// when a write occurs.
///
/// Generic parameters:
/// N: Number of LEDs (modules * colours)
/// M: Number of channels (colors) per module
/// H: Type of hardware driver - can usually be inferred
pub struct LEDs<const N: usize, const M: usize, H: GenericHardware<N>> {
    /// This will store the state of each LED, with one u8 per LED
    leds: [u8; N],
    /// The hardware device being used for output
    hw_dev: H,
}

impl<const N: usize, const M: usize, H: GenericHardware<N>> LEDs<N, M, H> {
    /// Constructor to initialise LEDs struct
    pub fn new(hw_dev: H) -> Self {
        Self {
            leds: [0; N],
            hw_dev,
        }
    }

    /// Sets the value of one node in the pre-encoded LED buffer
    pub fn set_node(&mut self, idx: usize, node: [u8; M]) {
        self.leds[(idx * M)..(idx * M + M)].copy_from_slice(&node);
    }

    /// Writes the currently stored buffer
    pub fn write(&mut self) -> Result<(), H::Error> {
        self.hw_dev.encode_and_write(&self.leds)
    }
}
