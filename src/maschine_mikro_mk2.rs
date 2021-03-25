use hidapi::HidDevice;

use super::controller::{
    Canvas, Colour, Controller, Error, Event, EventCallback, MonochromeCanvas, BLACK, WHITE,
};
use crate::controller::{Button, Direction};

const INPUT_BUFFER_SIZE: usize = 512;

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
// pub const LED_UNKNOWN: u8 = 0x4E;

// Buttons
pub const BUTTON_SHIFT: u8 = 0x00;
pub const BUTTON_ERASE: u8 = 0x01;
pub const BUTTON_REC: u8 = 0x02;
pub const BUTTON_PLAY: u8 = 0x03;
pub const BUTTON_GRID: u8 = 0x04;
pub const BUTTON_TRANSPORT_RIGHT: u8 = 0x05;
pub const BUTTON_TRANSPORT_LEFT: u8 = 0x06;
pub const BUTTON_RESTART: u8 = 0x07;
pub const BUTTON_MAIN_ENCODER: u8 = 0x0B;
pub const BUTTON_NOTE_REPEAT: u8 = 0x0C;
pub const BUTTON_SAMPLING: u8 = 0x0D;
pub const BUTTON_BROWSE: u8 = 0x0E;
pub const BUTTON_GROUP: u8 = 0x0F;
pub const BUTTON_MAIN: u8 = 0x10;
pub const BUTTON_BROWSE_RIGHT: u8 = 0x11;
pub const BUTTON_BROWSE_LEFT: u8 = 0x12;
pub const BUTTON_NAV: u8 = 0x13;
pub const BUTTON_CONTROL: u8 = 0x14;
pub const BUTTON_F3: u8 = 0x15;
pub const BUTTON_F2: u8 = 0x16;
pub const BUTTON_F1: u8 = 0x17;
pub const BUTTON_MUTE: u8 = 0x18;
pub const BUTTON_SOLO: u8 = 0x19;
pub const BUTTON_SELECT: u8 = 0x1A;
pub const BUTTON_DUPLICATE: u8 = 0x1B;
pub const BUTTON_VIEW: u8 = 0x1C;
pub const BUTTON_PAD_MODE: u8 = 0x1D;
pub const BUTTON_PATTERN: u8 = 0x1E;
pub const BUTTON_SCENE: u8 = 0x1F;
pub const BUTTON_NONE: u8 = 0x20;

///
/// Maschine Mikro Mk2 Controller
///
/// Requires a valid HID device
///
pub struct MaschineMikroMk2 {
    pub device: HidDevice,
    tick_state: u8,
    pub display: MonochromeCanvas,
    leds: [u8; 78],
    leds_dirty: bool,
    button_states: [bool; 45],
    shift_pressed: bool,
    pads_data: [u16; 16],
    pads_status: [bool; 16],
    encoder_value: u8,
    on_event: Option<EventCallback>,
}

impl MaschineMikroMk2 {
    pub const VENDOR_ID: u16 = 0x17cc;
    pub const PRODUCT_ID: u16 = 0x1200;

    pub fn new(device: HidDevice) -> Self {
        MaschineMikroMk2 {
            device,
            tick_state: 0,
            display: MonochromeCanvas::new(128, 64),
            leds: [0; 78],
            leds_dirty: true,
            button_states: [false; 45],
            shift_pressed: false,
            pads_data: [0; 16],
            pads_status: [false; 16],
            encoder_value: 0,
            on_event: None,
        }
    }

    /// Send a display frame for the graphics panel
    fn send_frame(&mut self) -> Result<(), Error> {
        if self.display.is_dirty() {
            for chunk in 0..4 {
                // TODO: need definition of this header.
                let mut buffer: Vec<u8> = vec![
                    0xE0,
                    0x00,
                    0x00,
                    (chunk * 2) as u8,
                    0x00,
                    0x80,
                    0x00,
                    0x02,
                    0x00,
                ];
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
            } else if (bytes_read > 0) && (buffer[0] == 0x20) && ((idx % 7) == 0) {
                self.process_pads(&buffer[1..])?;
            }
        }

        Ok(())
    }

    /// Process a buttons report message
    fn process_buttons(&mut self, buffer: &[u8]) -> Result<(), Error> {
        if buffer.len() < 5 {
            return Err(Error::InvalidReport);
        }

        // Scan buttons
        for btn in BUTTON_SHIFT..BUTTON_NONE {
            let button_pressed = is_button_pressed(&buffer, btn);
            if button_pressed != self.button_states[btn as usize] {
                self.button_states[btn as usize] = button_pressed;

                if btn == BUTTON_SHIFT {
                    self.shift_pressed = button_pressed;
                    self.set_led(LED_SHIFT, if button_pressed { WHITE } else { BLACK });
                } else if self.on_event.is_some() {
                    let button = self.as_device_button(btn);
                    self.on_event.as_mut().unwrap()(Event::ButtonChange(
                        button, button_pressed, self.shift_pressed,
                    ));
                }
            }
        }

        // Handle encoder data
        let encoder_value = buffer[4];
        if self.encoder_value != encoder_value {
            let direction = if ((self.encoder_value < encoder_value)
                | ((self.encoder_value == 0x0f) && (encoder_value == 0x00)))
                & (!((self.encoder_value == 0x00) & (encoder_value == 0x0f)))
            {
                Direction::Up
            } else {
                Direction::Down
            };
            self.encoder_value = encoder_value;
            if self.on_event.is_some() {
                self.on_event.as_mut().unwrap()(Event::EncoderChange(
                    0,
                    direction,
                    self.shift_pressed,
                ));
            }
        }

        Ok(())
    }

    /// Process a pads report message
    fn process_pads(&mut self, buffer: &[u8]) -> Result<(), Error> {
        if buffer.len() < 64 {
            return Err(Error::InvalidReport);
        }

        for idx in (0..32).step_by(2) {
            let low_byte = buffer[idx];
            let high_byte = buffer[idx + 1];
            let pad = ((high_byte & 0xF0) >> 4) as usize;
            let value = (((high_byte & 0x0F) as u16) << 8) | low_byte as u16;
            let pressed = value > 512;

            self.pads_data[pad] = value;
            if pressed | self.pads_status[pad] {
                self.pads_status[pad] = pressed;

                // Trigger if callback is defined
                if self.on_event.is_some() {
                    self.on_event.as_mut().unwrap()(Event::PadChange(
                        pad as u8,
                        if pressed { (value >> 4) as u8 } else { 0 },
                        self.shift_pressed,
                    ));
                }
            }
        }

        Ok(())
    }

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) {
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
    }

    /// Determine if an LED is RGB or Mono
    fn is_rgb_led(&self, led: u8) -> bool {
        (led == LED_GROUP) | (LED_PAD13..=LED_PAD04).contains(&led)
    }

    /// Convert a button code into a button enum
    fn as_device_button(&self, button: u8) -> Button {
        match button {
            BUTTON_ERASE => Button::Erase,
            BUTTON_REC => Button::Rec,
            BUTTON_PLAY => Button::Play,
            BUTTON_GRID => Button::Grid,
            BUTTON_TRANSPORT_RIGHT => Button::TransportRight,
            BUTTON_TRANSPORT_LEFT => Button::TransportLeft,
            BUTTON_RESTART => Button::Restart,
            BUTTON_MAIN_ENCODER => Button::MainEncoder,
            BUTTON_NOTE_REPEAT => Button::NoteRepeat,
            BUTTON_SAMPLING => Button::Sampling,
            BUTTON_BROWSE => Button::Browse,
            BUTTON_GROUP => Button::Group,
            BUTTON_MAIN => Button::Main,
            BUTTON_BROWSE_RIGHT => Button::BrowseRight,
            BUTTON_BROWSE_LEFT => Button::BrowseLeft,
            BUTTON_NAV => Button::Nav,
            BUTTON_CONTROL => Button::Control,
            BUTTON_F3 => Button::F3,
            BUTTON_F2 => Button::F2,
            BUTTON_F1 => Button::F1,
            BUTTON_MUTE => Button::Mute,
            BUTTON_SOLO => Button::Solo,
            BUTTON_SELECT => Button::Select,
            BUTTON_DUPLICATE => Button::Duplicate,
            BUTTON_VIEW => Button::View,
            BUTTON_PAD_MODE => Button::PadMode,
            BUTTON_PATTERN => Button::Pattern,
            BUTTON_SCENE => Button::Scene,
            _ => Button::Unknown,
        }
    }

    /// Convert a button into a LED index
    fn button_to_led(&self, button: Button) -> Option<u8> {
        match button {
            Button::Erase => Some(LED_ERASE),
            Button::Rec => Some(LED_REC),
            Button::Play => Some(LED_PLAY),
            Button::Grid => Some(LED_GRID),
            Button::TransportRight => Some(LED_TRANSPORT_RIGHT),
            Button::TransportLeft => Some(LED_TRANSPORT_LEFT),
            Button::Restart => Some(LED_RESTART),
            Button::NoteRepeat => Some(LED_NOTE_REPEAT),
            Button::Sampling => Some(LED_SAMPLING),
            Button::Browse => Some(LED_BROWSE),
            Button::Group => Some(LED_GROUP),
            Button::Main => Some(LED_MAIN),
            Button::BrowseRight => Some(LED_BROWSE_RIGHT),
            Button::BrowseLeft => Some(LED_BROWSE_LEFT),
            Button::Nav => Some(LED_NAV),
            Button::Control => Some(LED_CONTROL),
            Button::F3 => Some(LED_F3),
            Button::F2 => Some(LED_F2),
            Button::F1 => Some(LED_F1),
            Button::Mute => Some(LED_MUTE),
            Button::Solo => Some(LED_SOLO),
            Button::Select => Some(LED_SELECT),
            Button::Duplicate => Some(LED_DUPLICATE),
            Button::View => Some(LED_VIEW),
            Button::PadMode => Some(LED_PADMODE),
            Button::Pattern => Some(LED_PATTERN),
            Button::Scene => Some(LED_SCENE),
            _ => None,
        }
    }

    /// Convert a button into a LED index
    fn pad_to_led(&self, pad: u8) -> Option<u8> {
        match pad {
            0x0 => Some(LED_PAD13),
            0x1 => Some(LED_PAD14),
            0x2 => Some(LED_PAD15),
            0x3 => Some(LED_PAD16),
            0x4 => Some(LED_PAD09),
            0x5 => Some(LED_PAD10),
            0x6 => Some(LED_PAD11),
            0x7 => Some(LED_PAD12),
            0x8 => Some(LED_PAD05),
            0x9 => Some(LED_PAD06),
            0xA => Some(LED_PAD07),
            0xB => Some(LED_PAD08),
            0xC => Some(LED_PAD01),
            0xD => Some(LED_PAD02),
            0xE => Some(LED_PAD03),
            0xF => Some(LED_PAD04),
            _ => None,
        }
    }
}

impl Controller for MaschineMikroMk2 {
    fn set_button_led(&mut self, button: Button, colour: Colour) {
        match self.button_to_led(button) {
            Some(led) => self.set_led(led, colour),
            None => (),
        };
    }

    fn set_pad_led(&mut self, pad: u8, colour: Colour) {
        match self.pad_to_led(pad) {
            Some(led) => self.set_led(led, colour),
            None => (),
        };
    }

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

    fn on_event(&mut self, callback: EventCallback) {
        self.on_event = Some(callback);
    }
}

fn is_button_pressed(buffer: &[u8], button: u8) -> bool {
    let byte_idx = (button >> 3) as usize;
    (buffer[byte_idx] & (1 << (button % 8))) != 0
}
