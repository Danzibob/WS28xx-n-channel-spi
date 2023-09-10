//! Generic Hardware Abstraction Layer, no_std-compatible.

pub enum HardwareError {
    IoError,
    // Add more error types here as needed
}

/// Hardware device abstraction, which can be implemented by many different types of back-end (embedded, linux, etc.)
pub trait GenericHardware<const B: usize> {
    type Error;

    /// Initialise the device.
    fn init(&mut self);

    /// Sequentially write `byte_array` exactly as presented.
    fn write_raw(&mut self, byte_array: &[u8]) -> Result<(), Self::Error>;

    /// Encode `byte_array` suitable for WS81XX bitbanging, then write it.
    fn encode_and_write(&mut self, byte_array: &[u8]) -> Result<(), Self::Error>;
}
