//! The main goal of this crate was to work on Raspberry Pi. As the title says, this is bound
//! to `spi`-device for sending data. But you can use it also on other systems, i.e. embedded systems
//! (no_std-environments) but in these cases you must provide an own `encoding.rs` file if the
//! refresh rate doesn't match the value in [`timings::PI_SPI_HZ`].

#![no_std]

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "adapter_spidev")]
extern crate std;

pub mod generic_adapter; // generic [no_std] hardware abstraction
#[cfg(feature = "adapter_spidev")]
pub mod linux_spi; // specific [std]-implementation

// bound to Raspberry Pi SPI device but you can easily provide your own
// encoding functions.
pub mod linux_spi_encoding;
