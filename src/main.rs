mod controller;
mod maschine_mikro_mk2;

use controller::{Controller, Canvas, WHITE, BLACK};
use hidapi::HidApi;
use maschine_mikro_mk2::MaschineMikroMk2;
use crate::controller::{Button, Pixel, Event};


fn main() {
    let api = HidApi::new().unwrap();

    let mut ctlr = MaschineMikroMk2::new(
        api.open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID)
            .expect("Cannot open device"),
    );
    ctlr.display.fill(controller::Pixel::Off);

    let event_callback = |event: Event| {
        println!("{:?}", event);
        match event {
            Event::ButtonChange(button, _pressed, _shift) => {

            },
            Event::PadChange(_pad, _velocity, _shift) => {

            },
            Event::EncoderChange(_encoder, _pressed, _shift) => {

            }
        }
    };
    ctlr.on_event(Box::new(event_callback));

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
        ctlr.tick().unwrap();
    }
}
