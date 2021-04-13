use crate::colour::Colour;
pub use crate::error::Error;
use crate::events::{Button, EventTask};

///
/// Common controller behaviours
///
pub trait Controller: EventTask {
    ///
    /// Set the State of an Button LED
    ///
    fn set_button_led(&mut self, button: Button, colour: Colour);

    ///
    /// Set the colour of a pad
    ///
    fn set_pad_led(&mut self, pad: u8, colour: Colour);
}
