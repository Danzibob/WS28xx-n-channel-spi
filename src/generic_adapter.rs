//! Generic Hardware Abstraction Layer, no_std-compatible.
#[derive(Debug)]
pub enum HardwareError {
    IoError,
    // Add more error types here as needed
}

/// Hardware device abstraction, which can be implemented
/// by many different types of back-end (embedded, linux, etc.)
pub trait GenericHardware<const B: usize> {
    type Error;
    fn init(&mut self);
    fn write_raw(&mut self, byte_array: &[u8]) -> Result<(), Self::Error>;
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
    hw_dev: H
}

impl<const N: usize, const M: usize, H: GenericHardware<N>> LEDs<N, M, H> {
    /// Constructor to initialise LEDs struct
    pub fn new(mut hardware_device: H) -> Self {
        hardware_device.init();
        Self {
            leds: [0;N],
            hw_dev:hardware_device
        }
    }

    /// Sets the value of one node in the pre-encoded LED buffer
    pub fn set_node(&mut self, idx: usize, node: [u8;M]){
        self.leds[(idx*M)..(idx*M + M)].copy_from_slice(&node);
    }

    /// Writes the currently stored buffer
    pub fn write(&mut self) -> Result<(), H::Error> {
        self.hw_dev.encode_and_write(&self.leds)
    }
}