mod colour;
mod display;
mod error;

pub use colour::{
    Colour,
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
};

pub use display::{
    Display,
    MonochromeDisplay,
    Pixel,
};

pub use error::{
    Error,
};


///
/// Common controller behaviours
/// 
pub trait Controller {
    /// Perform any update events with the contoller device
    fn tick(&mut self) -> Result<(), Error>;

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) -> Result<(), Error>;

    /// Explicity turn an LED off (black)
    fn set_led_off(&mut self, led: u8) -> Result<(), Error> {
        self.set_led(led, BLACK)
    }

    /// The specified LED is an RGB led
    fn is_rgb_led(&self, led: u8) -> bool;
}
