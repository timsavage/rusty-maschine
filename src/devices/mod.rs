use hidapi::HidApi;

use crate::controller::Error;
use maschine_mikro_mk2::MaschineMikroMk2;

mod maschine_mikro_mk2;

pub fn get_device(hid_api: &HidApi) -> Result<MaschineMikroMk2, Error> {
    let device = hid_api
        .open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID)
        .expect("Cannot open device");

    println!(
        "Product: {}",
        device.get_product_string()?.unwrap_or_default()
    );
    println!(
        "Manufacturer: {}",
        device.get_manufacturer_string()?.unwrap_or_default()
    );
    println!(
        "Serial Number: {}",
        device.get_serial_number_string()?.unwrap_or_default()
    );

    Ok(MaschineMikroMk2::new(device))
}
