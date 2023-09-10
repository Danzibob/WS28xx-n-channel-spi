//! Adapter for SPI-dev on Linux-systems.
//!
//! This requires std.
use crate::generic_adapter::*;
use crate::linux_spi_encoding::*;
use crate::std::io;
use crate::std::io::Write;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};

/// Wrapper around Spidev.
pub struct LinuxSPI<const B: usize>
where
    [u8; B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
{
    spi: Spidev,
    buffer: [u8; B * SPI_BYTES_PER_BIT * BITS_PER_PX],
}

// Implement Hardware abstraction for device.
impl<const B: usize> GenericHardware<B> for LinuxSPI<B>
where
    [u8; B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
{
    type Error = std::io::Error;

    fn init(&mut self) {
        self.buffer = [0; B * SPI_BYTES_PER_BIT * BITS_PER_PX];
    }
    fn write_raw(&mut self, encoded_data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write_all(&encoded_data)
    }

    fn encode_and_write(&mut self, node_data: &[u8]) -> Result<(), Self::Error> {
        for (i, byte) in node_data.iter().flat_map(encode_pixel).enumerate() {
            self.buffer[i] = byte;
        }
        self.spi.write_all(&self.buffer)
    }
}

impl<const B: usize> LinuxSPI<B>
where
    [u8; B * SPI_BYTES_PER_BIT * BITS_PER_PX]:,
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
            buffer: [0; B * SPI_BYTES_PER_BIT * BITS_PER_PX],
        })
    }
}
