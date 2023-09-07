//! Generic Hardware Abstraction Layer, no_std-compatible.

/// Custom error type to return errors without strings
#[derive(Debug)]
pub enum HardwareError {
    WriteError(usize)
}

/// Hardware device abstraction, which can be implemented
/// by many different types of back-end (embedded, linux, etc.)
pub trait GenericHardware {
    fn init(&mut self, size: usize);
    fn write_raw(&mut self, byte_array: &[u8]) -> Result<(), HardwareError>;
    fn encode_and_write(&mut self, byte_array: &[u8]) -> Result<(), HardwareError>;
}

pub trait WS28xxAdapter<const N: usize, const M: usize, H: GenericHardware>{
    const LEDS_PER_NODE: usize;

    fn _get_hw_dev(&mut self) -> &mut H;
    fn set_node(&mut self, idx: usize, node: [u8;M]);
    fn _get_leds(&mut self) -> [u8;N];

    /// Writes a raw array of bytes to the output device
    /// Only to be used with pre-encoded frames of data
    fn write_raw(&mut self, byte_array: &[u8]) -> Result<(), HardwareError> {
        self._get_hw_dev().write_raw(&byte_array)
    }

    /// Writes the currently stored buffer
    fn write(&mut self) -> Result<(), HardwareError> {
        let array = self._get_leds();
        self._get_hw_dev().encode_and_write(&array)
    }
}

/// N: Number of LEDs (modules * colours)
/// M: Number of channels (colors) per module
/// H: Type of hardware driver - can usually be inferred
pub struct LEDs<const N: usize, const M: usize, H: GenericHardware> {
    /// This will store the state of each LED, with one u8 per LED
    leds: [u8; N],
    /// The hardware device being used for output
    hw_dev: H
}

/// Constructor to initialise LEDs struct
impl<const N: usize, const M: usize, H: GenericHardware> LEDs<N, M, H> {
    pub fn new(mut hardware_device: H) -> Self {
        hardware_device.init(N);
        Self {
            leds: [0;N],
            hw_dev:hardware_device
        }
    }
}

/// Implement the WS28xxAdapter trait for the LEDs struct
impl<const N: usize, const M: usize, H: GenericHardware> WS28xxAdapter<N,M,H> for LEDs<N,M,H> {
    const LEDS_PER_NODE: usize = N;
    fn _get_hw_dev(&mut self) -> &mut H{
        &mut self.hw_dev
    }
    fn set_node(&mut self, idx: usize, node: [u8;M]){
        for i in 0..M{
            self.leds[idx*M + i] = node[i];
        }
    }
    fn _get_leds(&mut self) -> [u8;N]{
        self.leds
    }
}