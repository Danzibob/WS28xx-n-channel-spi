// Uses rust bindings to the popular rpi-ws281x library

use crate::GenericHardware;
use rs_ws281x;

/// Colour information for a single arbitrary size node (e.g. one RGB unit)
pub type Node<const N: usize> = [u8; N];

pub fn setup<const N: usize>(
    num_nodes: usize,
    max_brightness: u8,
) -> Result<rs_ws281x::Controller, rs_ws281x::WS2811Error> {
    let num_leds = (num_nodes * N + 2) / 3;
    rs_ws281x::ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            rs_ws281x::ChannelBuilder::new()
                .pin(18) // GPIO 18
                .count(num_leds as i32) // Number of LEDs
                .strip_type(rs_ws281x::StripType::Ws2811Bgr)
                .brightness(max_brightness) // default: 255
                .build(),
        )
        .build()
}

// Implement Hardware abstraction for device.
impl<const B: usize> GenericHardware<B> for rs_ws281x::Controller {
    type Error = rs_ws281x::WS2811Error;

    fn write_raw(&mut self, encoded_data: &[u8]) -> Result<(), Self::Error> {
        let leds = self.leds_mut(0);
        assert!(encoded_data.len() == leds.len() * 3);
        for i in 0..leds.len() {
            leds[i].copy_from_slice(&encoded_data[(i * 4)..(i * 4 + 4)])
        }
        self.render()
    }

    fn encode_and_write(&mut self, nodes: &[u8]) -> Result<(), Self::Error> {
        let leds = self.leds_mut(0);
        for i in 0..leds.len() {
            leds[i][0..3].clone_from_slice(&nodes[(i * 3)..(i * 3 + 3)])
        }
        self.render()
    }
}
