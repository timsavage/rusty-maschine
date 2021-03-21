use hidapi::HidDevice;

use super::controller::{Colour, Controller};

const INPUT_BUFFER_SIZE: usize = 512;

// Endpoints
pub const EP_DISPLAY: u8 = 0x08;
pub const EP_OUTPUT: u8 = 0x01;
pub const EP_INPUT: u8 = 0x84;

// LEDs
pub const LED_GROUP: u8 = 0x08;
pub const LED_PAD13: u8 = 0x1E;
pub const LED_PAD04: u8 = 0x4B;

// Buttons
pub const BUTTON_SHIFT: u8 = 0x00;
pub const BUTTON_NONE: u8 = 0x20;

// #[repr(u8)]
// #[derive(Copy, Clone, Debug, Eq, PartialEq, int_enum::IntEnum)]
// pub enum Led {
//     F1 = 0x00,
//     F2 = 0x01,
//     F3 = 0x02,
//     Control = 0x03,
//     Nav = 0x04,
//     BrowseLeft = 0x05,
//     BrowseRight = 0x06,
//     Main = 0x07,
//     Group = 0x08, // GroupG = 0x09, GroupB = 0x0A,
//     Browse = 0x0B,
//     Sampling = 0x0C,
//     NoteRepeat = 0x0D,
//     Restart = 0x0E,
//     TransportLeft = 0x0F,
//     TransportRight = 0x10,
//     Grid = 0x11,
//     Play = 0x12,
//     Rec = 0x13,
//     Erase = 0x14,
//     Shift = 0x15,
//     Scene = 0x16,
//     Pattern = 0x17,
//     PadMode = 0x18,
//     View = 0x19,
//     Duplicate = 0x1A,
//     Select = 0x1B,
//     Solo = 0x1C,
//     Mute = 0x1D,

//     Pad13 = 0x1E, // Pad13G = 0x1F, Pad13B = 0x20,
//     Pad14 = 0x21, // Pad14G = 0x22, Pad14B = 0x23,
//     Pad15 = 0x24, // Pad15G = 0x25, Pad15B = 0x26,
//     Pad16 = 0x27, // Pad16G = 0x28, Pad16B = 0x29,
//     Pad9 = 0x2A,  // Pad9G = 0x2B,  Pad9B = 0x2C,
//     Pad10 = 0x2D, // Pad10G = 0x2E, Pad10B = 0x2F,
//     Pad11 = 0x30, // Pad11G = 0x31, Pad11B = 0x32,
//     Pad12 = 0x33, // Pad12G = 0x34, Pad12B = 0x35,
//     Pad5 = 0x36,  // Pad5G = 0x37,  Pad5B = 0x38,
//     Pad6 = 0x39,  // Pad6G = 0x3A,  Pad6B = 0x3B,
//     Pad7 = 0x3C,  // Pad7G = 0x3D,  Pad7B = 0x3E,
//     Pad8 = 0x3F,  // Pad8G = 0x40,  Pad8B = 0x41,
//     Pad1 = 0x42,  // Pad1G = 0x43,  Pad1B = 0x44,
//     Pad2 = 0x45,  // Pad2G = 0x46,  Pad2B = 0x47,
//     Pad3 = 0x48,  // Pad3G = 0x49,  Pad3B = 0x4A,
//     Pad4 = 0x4B,  // Pad4G = 0x4C,  Pad4B = 0x4D,

//     Unknown = 0x4E,
// }

// #[repr(u8)]
// #[derive(Copy, Clone, Debug, Eq, PartialEq, int_enum::IntEnum)]
// pub enum Button {
//     Shift = 0x00,
//     Erase = 0x01,
//     Rec = 0x02,
//     Play = 0x03,
//     Grid = 0x04,
//     TransportRight = 0x05,
//     TransportLeft = 0x06,
//     Restart = 0x07,

//     MainEncoder = 0x0B,
//     NoteRepeat = 0x0C,
//     Sampling = 0x0D,
//     Browse = 0x0E,
//     Group = 0x0F,

//     Main = 0x10,
//     BrowseRight = 0x11,
//     BrowseLeft = 0x12,
//     Nav = 0x13,
//     Control = 0x14,
//     F3 = 0x15,
//     F2 = 0x16,
//     F1 = 0x17,

//     Mute = 0x18,
//     Solo = 0x19,
//     Select = 0x1A,
//     Duplicate = 0x1B,
//     View = 0x1C,
//     PadMode = 0x1D,
//     Pattern = 0x1E,
//     Scene = 0x1F,

//     None = 0x20,
// }

pub struct MaschineMikroMk2 {
    pub device: HidDevice,

    tick_state: u8,

    leds: [u8; 78],
    leds_dirty: bool,

    button_states: [bool; 45],

    pads_data: [u16; 16],
    pads_status: [bool; 16],

    encoder_value: u8,
}

impl MaschineMikroMk2 {
    pub fn new(device: HidDevice) -> Result<Self, ()> {
        Ok(MaschineMikroMk2 {
            device: device,
            tick_state: 0,
            leds: [0; 78],
            leds_dirty: true,
            button_states: [false; 45],
            pads_data: [0; 16],
            pads_status: [false; 16],
            encoder_value: 0,
        })
    }

    /// Write updates to device
    fn send_leds(&mut self) -> Result<(), ()> {
        if self.leds_dirty {
            let mut buffer: Vec<u8> = vec![0x80];
            buffer.extend_from_slice(&self.leds);
            self.device.write(buffer.as_slice()).unwrap();
        }
        self.leds_dirty = false;

        Ok(())
    }

    fn read(&mut self) -> Result<(), ()> {
        let mut buffer = [0u8; INPUT_BUFFER_SIZE];

        for idx in 0..32 {
            let bytes_read = match self.device.read(&mut buffer) {
                Ok(n) => n,
                Err(_) => return Err(()),
            };
            if bytes_read > 0 && buffer[0] == 0x01 {
                self.process_buttons(&buffer[1..6])?;
            }
            else if (bytes_read > 0) && (buffer[0] == 0x20) && ((idx % 7) == 0) {
                self.process_pads(&buffer[1..])?;
            }
        }

        Ok(())
    }

    fn process_buttons(&mut self, buffer: &[u8]) -> Result<(), ()> {
        let shift_pressed = self.is_button_pressed(&buffer, BUTTON_SHIFT)?;

        // Scan buttons 
        for btn in 1..BUTTON_NONE {
            let button_pressed = self.is_button_pressed(&buffer, btn)?;
            if button_pressed != self.button_states[btn as usize] {
                self.button_states[btn as usize] = button_pressed;
                self.on_button_change(btn, button_pressed, shift_pressed);
            }
        }

        // Handle encoder data
        let encoder_value = buffer[4];
        if self.encoder_value != encoder_value {
            let direction_up =
                ((self.encoder_value < encoder_value) | ((self.encoder_value == 0x0f) && (encoder_value == 0x00)))
                & (!((self.encoder_value == 0x00) & (encoder_value == 0x0f)));
            self.encoder_value = encoder_value;
            self.on_encoder_change(0, direction_up, shift_pressed);
        }

        Ok(())
    }

    fn on_button_change(&self, button: u8, pressed: bool, shift: bool) {
        println!("Button: {}; Pressed: {}; Shift: {}", button, pressed, shift);
    }

    fn is_button_pressed(&self, buffer: &[u8], button: u8) -> Result<bool, ()> {
        let byte_idx = (button >> 3) as usize;
        Ok((buffer[byte_idx] & (1 << (button % 8))) != 0)
    }

    fn on_encoder_change(&self, _encoder: u8, direction: bool, shift: bool) {
        println!("Encoder: Direction: {}; Shift: {}", direction, shift);
    }

    fn process_pads(&mut self, buffer: &[u8]) -> Result<(), ()> {
        for idx in (0..64).step_by(2) {
            let low_byte = buffer[idx];
            let high_byte = buffer[idx + 1];
            let pad = ((high_byte & 0xF0) >> 4) as usize;
            self.pads_data[pad] = (((high_byte & 0x0F) as u16) << 8) | low_byte as u16;

            if self.pads_data[pad] > 200 {
                self.pads_status[pad] = true;
                self.on_pad_changed(pad as u8, (self.pads_data[pad] >> 4) as u8);
            } else {
                if self.pads_status[pad]
                {
                    self.pads_status[pad] = false;
                    self.on_pad_changed(pad as u8, 0);
                }
            }
        }

        Ok(())
    }

    fn on_pad_changed(&self, pad: u8, velocity: u8) {
        println!("Pad: {}; Velocity: {}", pad, velocity);
    }
}

impl Controller for MaschineMikroMk2 {
    fn tick(&mut self) -> Result<(), ()> {
        if self.tick_state == 0 {
        } else if self.tick_state == 1 {
            self.send_leds()?;
        } else if self.tick_state == 2 {
            self.read()?;
        }

        self.tick_state = (self.tick_state + 1) % 3;

        Ok(())
    }

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) -> Result<(), ()> {
        let base = led as usize;

        if self.is_rgb_led(led) {
            let (r, g, b) = colour.components();

            self.leds_dirty =
                (r != self.leds[base]) | (g != self.leds[base + 1]) | (b != self.leds[base + 2]);

            self.leds[base] = r;
            self.leds[base + 1] = g;
            self.leds[base + 2] = b;
        } else {
            let m = colour.as_mono();
            self.leds_dirty = m != self.leds[base];
            self.leds[base] = m;
        }

        Ok(())
    }

    /// Determine if an LED is RGB or Mono
    fn is_rgb_led(&self, led: u8) -> bool {
        (led == LED_GROUP) | ((led >= LED_PAD13) & (led <= LED_PAD04))
    }
}
