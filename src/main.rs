mod controller;
mod maschine_mikro_mk2;

use controller::{Controller, Display};
use hidapi::HidApi;
use maschine_mikro_mk2::MaschineMikroMk2;

fn main() {
    let api = HidApi::new().unwrap();

    let mut ctlr = MaschineMikroMk2::new(
        api.open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID).expect("Cannot open device")
    );
    ctlr.display.invert();

    println!(
        "Device Product: {}",
        ctlr.device.get_product_string().unwrap().unwrap_or_default()
    );
    println!(
        "Device Manufacturer: {}",
        ctlr.device.get_manufacturer_string().unwrap().unwrap_or_default()
    );
    println!(
        "Device Serial Number: {}",
        ctlr.device.get_serial_number_string().unwrap().unwrap_or_default()
    );

    loop {
        ctlr.tick().unwrap();
    }
}
