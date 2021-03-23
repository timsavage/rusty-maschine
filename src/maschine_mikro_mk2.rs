use hidapi::HidDevice;

use super::controller::{
    Controller, 
    Display,
    MonochromeDisplay, 
    Error, 
    Colour, 
    WHITE, 
    BLACK
};

const INPUT_BUFFER_SIZE: usize = 512;

// Endpoints
// pub const EP_DISPLAY: u8 = 0x08;
// pub const EP_OUTPUT: u8 = 0x01;
// pub const EP_INPUT: u8 = 0x84;

// LEDs
pub const LED_F1: u8 = 0x00;
pub const LED_F2: u8 = 0x01;
pub const LED_F3: u8 = 0x02;
pub const LED_CONTROL: u8 = 0x03;
pub const LED_NAV: u8 = 0x04;
pub const LED_BROWSE_LEFT: u8 = 0x05;
pub const LED_BROWSE_RIGHT: u8 = 0x06;
pub const LED_MAIN: u8 = 0x07;
pub const LED_GROUP: u8 = 0x08;
pub const LED_BROWSE: u8 = 0x0B;
pub const LED_SAMPLING: u8 = 0x0C;
pub const LED_NOTE_REPEAT: u8 = 0x0D;
pub const LED_RESTART: u8 = 0x0E;
pub const LED_TRANSPORT_LEFT: u8 = 0x0F;
pub const LED_TRANSPORT_RIGHT: u8 = 0x10;
pub const LED_GRID: u8 = 0x11;
pub const LED_PLAY: u8 = 0x12;
pub const LED_REC: u8 = 0x13;
pub const LED_ERASE: u8 = 0x14;
pub const LED_SHIFT: u8 = 0x15;
pub const LED_SCENE: u8 = 0x16;
pub const LED_PATTERN: u8 = 0x17;
pub const LED_PADMODE: u8 = 0x18;
pub const LED_VIEW: u8 = 0x19;
pub const LED_DUPLICATE: u8 = 0x1A;
pub const LED_SELECT: u8 = 0x1B;
pub const LED_SOLO: u8 = 0x1C;
pub const LED_MUTE: u8 = 0x1D;
pub const LED_PAD13: u8 = 0x1E;
pub const LED_PAD14: u8 = 0x21;
pub const LED_PAD15: u8 = 0x24;
pub const LED_PAD16: u8 = 0x27;
pub const LED_PAD09: u8 = 0x2A;
pub const LED_PAD10: u8 = 0x2D;
pub const LED_PAD11: u8 = 0x30;
pub const LED_PAD12: u8 = 0x33;
pub const LED_PAD05: u8 = 0x36;
pub const LED_PAD06: u8 = 0x39;
pub const LED_PAD07: u8 = 0x3C;
pub const LED_PAD08: u8 = 0x3F;
pub const LED_PAD01: u8 = 0x42;
pub const LED_PAD02: u8 = 0x45;
pub const LED_PAD03: u8 = 0x48;
pub const LED_PAD04: u8 = 0x4B;
pub const LED_UNKNOWN: u8 = 0x4E;

// Buttons
pub const BUTTON_SHIFT: u8 = 0x00;
pub const BUTTON_NONE: u8 = 0x20;




///
/// Maschine Mikro Mk2 Controller 
/// 
/// Requires a valid HID device
/// 
pub struct MaschineMikroMk2 {
    pub device: HidDevice,
    tick_state: u8,
    pub display: MonochromeDisplay,
    leds: [u8; 78],
    leds_dirty: bool,
    button_states: [bool; 45],
    shift_pressed: bool,
    pads_data: [u16; 16],
    pads_status: [bool; 16],
    encoder_value: u8,
}

impl MaschineMikroMk2 {
    pub const VENDOR_ID: u16 = 0x17cc;
    pub const PRODUCT_ID: u16 = 0x1200;

    pub fn new(device: HidDevice) -> Self {
        MaschineMikroMk2 {
            device,
            tick_state: 0,
            display: MonochromeDisplay::new(128, 64),
            leds: [0; 78],
            leds_dirty: true,
            button_states: [false; 45],
            shift_pressed: false,
            pads_data: [0; 16],
            pads_status: [false; 16],
            encoder_value: 0,
        }
    }

    /// Send a display frame for the graphics panel
    fn send_frame(&mut self) -> Result<(), Error> {
        if self.display.is_dirty() {
            for chunk in 0..4 {
                // TODO: need definition of this header.
                let mut buffer: Vec<u8> = vec![0xE0, 0x00, 0x00, (chunk * 2) as u8, 0x00, 0x80, 0x00, 0x02, 0x00];
                let x_offset = chunk * 256;
                buffer.extend_from_slice(&self.display.data()[x_offset..(x_offset + 256)]);
                self.device.write(buffer.as_slice())?;
            }
        }
        self.display.clear_dirty_flag();

        Ok(())
    }

    /// Update LEDs if the array has been updated
    fn send_leds(&mut self) -> Result<(), Error> {
        if self.leds_dirty {
            let mut buffer: Vec<u8> = vec![0x80];
            buffer.extend_from_slice(&self.leds);
            self.device.write(buffer.as_slice())?;
        }
        self.leds_dirty = false;

        Ok(())
    }

    /// Read incoming reports from the device
    fn read(&mut self) -> Result<(), Error> {
        let mut buffer = [0u8; INPUT_BUFFER_SIZE];

        for idx in 0..32 {
            let bytes_read = match self.device.read(&mut buffer) {
                Ok(n) => n,
                Err(e) => return Err(Error::HidAPI(e)),
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

    /// Process a buttons report message
    fn process_buttons(&mut self, buffer: &[u8]) -> Result<(), Error> {
        if buffer.len() < 5 {
            return Err(Error::BufferUnderrun);
        }

        // Scan buttons 
        for btn in BUTTON_SHIFT..BUTTON_NONE {
            let button_pressed = is_button_pressed(&buffer, btn);
            if button_pressed != self.button_states[btn as usize] {
                self.button_states[btn as usize] = button_pressed;

                if btn == BUTTON_SHIFT {
                    self.shift_pressed = button_pressed;
                    self.set_led(LED_SHIFT, if button_pressed { WHITE } else { BLACK } )?;
                } else {
                    self.on_button_change(btn, button_pressed, self.shift_pressed);
                }
            }
        }

        // Handle encoder data
        let encoder_value = buffer[4];
        if self.encoder_value != encoder_value {
            let direction_up =
                ((self.encoder_value < encoder_value) | ((self.encoder_value == 0x0f) && (encoder_value == 0x00)))
                & (!((self.encoder_value == 0x00) & (encoder_value == 0x0f)));
            self.encoder_value = encoder_value;
            self.on_encoder_change(0, direction_up, self.shift_pressed);
        }

        Ok(())
    }

    /// Process a pads report message
    fn process_pads(&mut self, buffer: &[u8]) -> Result<(), Error> {
        if buffer.len() < 64 {
            return Err(Error::BufferUnderrun);
        }

        for idx in (0..32).step_by(2) {
            let low_byte = buffer[idx];
            let high_byte = buffer[idx + 1];
            let pad = ((high_byte & 0xF0) >> 4) as usize;
            let value = (((high_byte & 0x0F) as u16) << 8) | low_byte as u16;
            self.pads_data[pad] = value;

            if value > 512 {
                self.pads_status[pad] = true;
                self.on_pad_changed(pad as u8, (value >> 4) as u8, self.shift_pressed);
            } else if self.pads_status[pad] {
                self.pads_status[pad] = false;
                self.on_pad_changed(pad as u8, 0, self.shift_pressed);
            }
        }

        Ok(())
    }

    fn on_button_change(&self, button: u8, pressed: bool, shift: bool) {
        println!("Button: {}; Pressed: {}; Shift: {}", button, pressed, shift);
    }
    
    fn on_encoder_change(&self, _encoder: u8, direction: bool, shift: bool) {
        println!("Encoder: Direction: {}; Shift: {}", direction, shift);
    }

    fn on_pad_changed(&mut self, pad: u8, velocity: u8, shift: bool) {
        println!("Pad: {}; Velocity: {}; Shift: {}", pad, velocity, shift);

        let led = match pad {
            0xC => LED_PAD01,
            0xD => LED_PAD02,
            0xE => LED_PAD03,
            0xF => LED_PAD04,
            0x8 => LED_PAD05,
            0x9 => LED_PAD06,
            0xA => LED_PAD07,
            0xB => LED_PAD08,
            0x4 => LED_PAD09,
            0x5 => LED_PAD10,
            0x6 => LED_PAD11,
            0x7 => LED_PAD12,
            0x0 => LED_PAD13,
            0x1 => LED_PAD14,
            0x2 => LED_PAD15,
            0x3 => LED_PAD16,
            _ => LED_UNKNOWN,
        };

        if led != LED_UNKNOWN {
            let colour = if shift {
                super::controller::WHITE
            } else { 
                match pad % 6 {
                    0 => Colour::new(velocity, 0x00, 0x00),
                    1 => Colour::new(velocity, velocity, 0x00),
                    2 => Colour::new(0x00, velocity, 0x00),
                    3 => Colour::new(0x00, velocity, velocity),
                    4 => Colour::new(0x00, 0x00, velocity),
                    5 => Colour::new(velocity, 0x00, velocity),
                    _ => super::controller::BLACK,
                }
            };

            self.set_led(led, colour).ok();
        }
    }
}

impl Controller for MaschineMikroMk2 {
    fn tick(&mut self) -> Result<(), Error> {
        if self.tick_state == 0 {
            self.send_frame()?;
        } else if self.tick_state == 1 {
            self.send_leds()?;
        } else if self.tick_state == 2 {
            self.read()?;
        }

        self.tick_state = (self.tick_state + 1) % 3;

        Ok(())
    }

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) -> Result<(), Error> {
        let base = led as usize;

        if self.is_rgb_led(led) {
            let (r, g, b) = colour.components();

            self.leds_dirty =
                (r != self.leds[base]) | (g != self.leds[base + 1]) | (b != self.leds[base + 2]);

            self.leds[base] = r >> 1;
            self.leds[base + 1] = g >> 1;
            self.leds[base + 2] = b >> 1;
        } else {
            let m = colour.as_mono();
            self.leds_dirty = m != self.leds[base];
            self.leds[base] = m;
        }

        Ok(())
    }

    /// Determine if an LED is RGB or Mono
    fn is_rgb_led(&self, led: u8) -> bool {
        (led == LED_GROUP) | (LED_PAD13..=LED_PAD04).contains(&led)
    }
}


fn is_button_pressed(buffer: &[u8], button: u8) -> bool {
    let byte_idx = (button >> 3) as usize;
    (buffer[byte_idx] & (1 << (button % 8))) != 0
}

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