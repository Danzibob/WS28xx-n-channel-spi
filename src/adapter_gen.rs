//! Generic Hardware Abstraction Layer, no_std-compatible.

use crate::encoding::*;
use alloc::boxed::Box;
use alloc::string::String;

/// SPI-device abstraction.
pub trait HardwareDev {
    type Error; 
    fn write_all(&mut self, encoded_data: &[u8]) -> Result<(), Self::Error>;
}

pub trait WS28xxAdapter {
    /// Returns a reference to the hardware device.
    /// This function only needs to be implemented once in the generic adapter.
    fn get_hw_dev(&mut self) -> &mut Box<dyn HardwareDev<Error=String>>;

    /// Encodes LED values and write them via the hardware device to the LEDs. The length of the vector
    /// is the number of LEDs you want to write to. *Note* that if you have performance critical
    /// applications (like you need a signal on the LEDS on a given time) it's a better idea
    /// to encode the data earlier by yourself using [`crate::encoding`]-module 
    /// and calling [`WS28xxAdapter::write_encoded_slice`].
    fn write_slice(&mut self, data: &[u8]) -> Result<(), String> {
        let encoded_data = encode_slice(data);
        self.write_encoded_slice(&encoded_data)
    }

    /// Clears all LEDs. Sets each to (0, 0, 0).
    fn clear(&mut self, num_leds: usize) {
        let data = vec![0; num_leds];
        self.write_slice(&data).unwrap();
    }

    /// Directly writes encoded LED values via hardware device to the LEDs. This method and the encoded data
    /// must fulfill the restrictions given by [`crate::timings`] and [`crate::encoding`] if the hardware
    /// device uses the specified frequency in [`crate::timings::PI_SPI_HZ`].
    fn write_encoded_slice(&mut self, encoded_data: &[u8]) -> Result<(), String> {
        self.get_hw_dev().write_all(&encoded_data)
            .map_err(|_| {
                format!(
                    "Failed to send {} bytes via the specified hardware device. If you use SPI on Linux Perhaps your SPI buffer is too small!\
                     Check https://www.raspberrypi.org/forums/viewtopic.php?p=309582#p309582 for example.",
                    encoded_data.len()
                )}
            )
    }
}

/// Platform agnostic (generic) adapter that connects your application via your specified
/// hardware interface to your WS28xx LEDs. *Handle this as something like an abstract class
/// for concrete implementations!* This works in `#[no-std]`-environments.
pub struct WS28xxGenAdapter {
    hw: Box<dyn HardwareDev<Error=String>>,
}

impl WS28xxGenAdapter {
    /// Constructor that stores the hardware device in the adapter.
    pub fn new(hw: Box<dyn HardwareDev<Error=String>>) -> Self {
        Self { hw }
    }
}

// Implement the getter for the hardware device.
impl WS28xxAdapter for WS28xxGenAdapter {
    fn get_hw_dev(&mut self) -> &mut Box<dyn HardwareDev<Error=String>> {
        &mut self.hw
    }
}
