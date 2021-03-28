mod controller;
mod logo;
mod maschine_mikro_mk2;

use hidapi::HidApi;
use controller::{Controller, Canvas, MonochromeCanvas, WHITE, BLACK, Button, Pixel, Event};
use maschine_mikro_mk2::MaschineMikroMk2;


fn main() {
    let logo = MonochromeCanvas::from_buffer(128, 64, &logo::LOGO);

    let api = HidApi::new().unwrap();

    let mut ctlr = MaschineMikroMk2::new(
        api.open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID)
            .expect("Cannot open device"),
    );
    ctlr.display.fill(controller::Pixel::Off);
    ctlr.display.copy_from(&logo);
    ctlr.set_button_led(Button::F1, WHITE);
    ctlr.set_button_led(Button::F2, WHITE);
    ctlr.set_button_led(Button::F3, WHITE);
    ctlr.set_button_led(Button::Nav, WHITE);
    ctlr.set_button_led(Button::BrowseLeft, WHITE);
    ctlr.set_button_led(Button::BrowseRight, WHITE);

    ctlr.set_pad_led(0, controller::GREEN);
    ctlr.set_pad_led(3, controller::GREEN);
    ctlr.set_pad_led(12, controller::GREEN);
    ctlr.set_pad_led(15, controller::GREEN);
    ctlr.set_pad_led(0, controller::GREEN);
    ctlr.set_pad_led(5, controller::RED);
    ctlr.set_pad_led(6, controller::RED);
    ctlr.set_pad_led(9, controller::BLUE);
    ctlr.set_pad_led(10, controller::BLUE);


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
