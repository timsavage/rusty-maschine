use hidapi::HidApi;

use colour::Colour;
use controller::{Canvas, Controller, MonochromeCanvas, Pixel};
use devices::get_device;
use events::{Button, Direction, Event, EventContext, EventTask};
use gui::logo;

mod colour;
mod controller;
mod devices;
mod error;
mod events;
mod gui;

fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();

    let logo = MonochromeCanvas::from_buffer(128, 64, &logo::LOGO);
    ctlr.display.fill(controller::Pixel::Off);
    ctlr.display.copy_from(&logo);
    ctlr.display.print("Hello World", 7, 0, Pixel::On);

    let mut count = 0i8;

    loop {
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            match event {
                Event::ButtonChange(button, pressed, shift) => {
                    match button {
                        Button::MainEncoder => count = 0,
                        Button::F1 => if pressed {
                            ctlr.display.invert();
                        }
                        Button::F2 => if pressed {
                            ctlr.display.fill(Pixel::On);
                        }
                        Button::F3 => if pressed {
                            ctlr.display.fill(Pixel::Off);
                        }
                        Button::Nav => if pressed {
                            ctlr.display.fill(Pixel::Off);
                            for idx in 0..96 {
                                let col = (idx % 21) * 6;
                                let row = idx / 21;
                                ctlr.display.print_char((idx as u8 + 0x20) as char, row, col, Pixel::On);
                            }
                        }
                        _ => {}
                    };
                    ctlr.set_button_led(
                        button,
                        if pressed | shift {
                            Colour::WHITE
                        } else {
                            Colour::BLACK
                        },
                    );
                }
                Event::PadChange(pad, velocity, shift) => {
                    let colour = if shift {
                        Colour::WHITE
                    } else {
                        Colour::new(velocity, 0, 0)
                    };
                    ctlr.set_pad_led(pad, colour);
                }
                Event::EncoderChange(_encoder, dir, shift) => {
                    count += match dir {
                        Direction::Up => 1,
                        Direction::Down => -1,
                    } * if shift { 10 } else { 1 };
                    let value = format!("{:03}", count);
                    ctlr.display.print(value.as_str(), 0, 0, Pixel::Off);
                }
            }
        }
    }
}
