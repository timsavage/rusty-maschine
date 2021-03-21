pub mod controller;
mod maschine_mikro_mk2;

use controller::{Colour, Controller};
use hidapi::HidApi;
use maschine_mikro_mk2::MaschineMikroMk2;

const VID: u16 = 0x17cc;
const PID: u16 = 0x1200; // Maschine Mikro V2

fn main() {
    let api = HidApi::new().unwrap();

    let mut ctlr = MaschineMikroMk2::new(api.open(VID, PID).expect("Cannot open device")).unwrap();

    println!(
        "Device Product: {}",
        ctlr.device.get_product_string().unwrap().unwrap()
    );
    println!(
        "Device Manufacturer: {}",
        ctlr.device.get_manufacturer_string().unwrap().unwrap()
    );
    println!(
        "Device Serial Number: {}",
        ctlr.device.get_serial_number_string().unwrap().unwrap()
    );

    let mut last_update = std::time::Instant::now();
    let one_sec = std::time::Duration::from_millis(100);
    let mut idx = 0x1E;
    loop {
        if last_update.elapsed() >= one_sec {
            // idx += 3;
            // if idx > 0x4B { idx = 0x1E }
            // controller.set_led(idx, Colour::random()).ok();

            ctlr.set_led_off(idx).ok();
            if ctlr.is_rgb_led(idx) {
                idx = (idx + 3) % 0x4B;
            } else {
                idx = (idx + 1) % 0x4B;
            }

            if ctlr.is_rgb_led(idx) {
                ctlr.set_led(idx, Colour::random()).ok();
            } else {
                ctlr.set_led(idx, Colour::on()).ok();
            }

            last_update = std::time::Instant::now();
        }

        ctlr.tick().unwrap();
    }
}
