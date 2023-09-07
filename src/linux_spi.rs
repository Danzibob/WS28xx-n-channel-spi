//! Adapter for SPI-dev on Linux-systems. This requires std.
use crate::generic_adapter::*;
use crate::linux_spi_encoding::*;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use crate::std::io;
use crate::std::io::Write;
use std::vec;
use std::vec::Vec;

/// Wrapper around Spidev.
pub struct LinuxSPI{
    spi: Spidev,
    buffer: Vec<u8>
}

// Implement Hardware abstraction for device.
impl GenericHardware for LinuxSPI{
    fn init(&mut self, num_leds: usize){
        let size = num_leds * SPI_BYTES_PER_BIT * BITS_PER_PX;
        self.buffer = vec![0; size];
    }
    fn write_raw(&mut self, encoded_data: &[u8]) -> Result<(), HardwareError> {
        self.spi.write_all(&encoded_data)
            .map_err(|_| HardwareError::WriteError(encoded_data.len()) )
    }

    fn encode_and_write(&mut self, node_data: &[u8]) -> Result<(), HardwareError>{
        let mut buffer_idx = 0;
        for px in node_data{
            let encoded_byte = encode_pixel(px);
            for byte in encoded_byte{
                self.buffer[buffer_idx] = byte;
                buffer_idx += 1;
            }
        }
        self.buffer = node_data.iter().flat_map(encode_pixel).collect();
        self.spi.write_all(&self.buffer)
            .map_err(|_| HardwareError::WriteError(self.buffer.len()) )
    }
}

impl LinuxSPI {
    /// Connects your application with the SPI-device of your device.
    /// This uses the `spidev`-crate. Returns a new adapter object
    /// for the WS28xx LEDs.
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
        Ok(Self{
            spi,
            buffer: Vec::new()
        })
    }
}