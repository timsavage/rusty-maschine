mod colour;
mod display;
mod error;
mod constants;

pub use colour::{Colour, BLACK, BLUE, GREEN, RED, WHITE};
pub use constants::{Button, Direction};
pub use display::{Display, MonochromeDisplay, Pixel};
pub use error::Error;

pub type OnButtonChange = fn(button: Button, pressed: bool, shift: bool);
pub type OnEncoderChange = fn(encoder: u8, direction: Direction, shift: bool);

///
/// Common controller behaviours
///
pub trait Controller {

    ///
    /// Set the State of an Button LED
    ///
    fn set_button_led(&mut self, button: Button, colour: Colour);

    ///
    /// Set the colour of a pad
    ///
    fn set_pad_led(&mut self, pad: u8, colour: Colour);

    /// Perform any update events with the controller device
    fn tick(&mut self) -> Result<(), Error>;

}
