use hidapi::HidApi;

use colour::Colour;
use controller::Controller;
use devices::get_device;
use events::{Button, Event, EventContext, EventTask};
use gui::logo;
use gui::ui::TextPanel;
use gui::display::{MonochromeCanvas, Canvas, Pixel};
use crate::gui::ui::{Control, EventHandler, Surface};

mod colour;
mod controller;
mod devices;
mod error;
mod events;
mod gui;

fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();
    // let logo = MonochromeCanvas::from_buffer(128, 64, &logo::LOGO);

    let mut surface: Surface<TextPanel> = Surface::new(ctlr.display.height(), ctlr.display.width());

    let mut panel = TextPanel::new();
    panel.set_text("This\nis some\ntext\n\nHello\nWorld\n\nThis is a new\nworld\nof\nstuff");
    surface.set_child(panel);

    loop {
        surface.paint(&mut ctlr.display);

        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            surface.handle(&event);

            match event {
                Event::ButtonChange(button, pressed, shift) => {
                    match button {
                        Button::F1 => {
                            if pressed {
                                ctlr.display.invert();
                            }
                        }
                        Button::F2 => {
                            if pressed {
                                ctlr.display.fill(Pixel::On);
                            }
                        }
                        Button::F3 => {
                            if pressed {
                                ctlr.display.fill(Pixel::Off);
                            }
                        }
                        Button::Nav => {
                            if pressed {
                                ctlr.display.fill(Pixel::Off);
                                for idx in 0..96 {
                                    let col = (idx % 21) * 6;
                                    let row = idx / 21;
                                    ctlr.display.print_char(
                                        (idx as u8 + 0x20) as char,
                                        row,
                                        col,
                                        Pixel::On,
                                    );
                                }
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
                // Event::EncoderChange(_encoder, dir, _shift) => {
                //     ctlr.display.vscroll_rows(0, 7, dir);
                // }
                _ => {}
            }
        }
    }
}
