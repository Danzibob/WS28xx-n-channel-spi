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

pub mod generic_adapter; // generic [no_std] hardware abstraction
#[cfg(feature = "adapter_spidev")]
pub mod linux_spi; // specific [std]-implementation

// Raspberry Pi SPI device
// you can easily provide your own encoding functions.
pub mod linux_spi_encoding;
