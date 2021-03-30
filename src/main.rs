use hidapi::HidApi;

use crate::controller::{Colour, Controller, BLACK, WHITE};
use controller::{Canvas, Event, EventContext, EventTask, MonochromeCanvas};
use maschine_mikro_mk2::MaschineMikroMk2;
mod controller;
mod logo;
mod maschine_mikro_mk2;

fn main() {
    let logo = MonochromeCanvas::from_buffer(128, 64, &logo::LOGO);
    let api = HidApi::new().unwrap();
    let mut ctlr = MaschineMikroMk2::new(
        api.open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID)
            .expect("Cannot open device"),
    );
    ctlr.display.fill(controller::Pixel::Off);
    ctlr.display.copy_from(&logo);

    println!(
        "Device Product: {}",
        ctlr.device
            .get_product_string()
            .unwrap()
            .unwrap_or_default()
    );
    println!(
        "Device Manufacturer: {}",
        ctlr.device
            .get_manufacturer_string()
            .unwrap()
            .unwrap_or_default()
    );
    println!(
        "Device Serial Number: {}",
        ctlr.device
            .get_serial_number_string()
            .unwrap()
            .unwrap_or_default()
    );

    loop {
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            match event {
                Event::ButtonChange(button, pressed, shift) => {
                    ctlr.set_button_led(button, if pressed | shift { WHITE } else { BLACK });
                }
                Event::PadChange(pad, velocity, shift) => {
                    let colour = if shift {
                        WHITE
                    } else {
                        Colour::new(velocity, 0, 0)
                    };
                    ctlr.set_pad_led(pad, colour);
                }
                _ => {}
            }
        }
    }
}
