use std::collections::VecDeque;

pub use colour::{Colour, BLACK, BLUE, GREEN, RED, WHITE};
pub use constants::{Button, Direction, Event};
pub use display::{Canvas, MonochromeCanvas, Pixel};
pub use error::Error;

mod colour;
mod constants;
mod display;
mod error;
mod font;

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
}

///
/// Context object for adding events
///
pub struct EventContext {
    pub events: VecDeque<Event>,
}

impl EventContext {
    pub fn new() -> Self {
        EventContext {
            events: VecDeque::new()
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

///
/// Generator for events
///
pub trait EventTask {
    ///
    /// Perform any update events with the controller device
    ///
    fn tick(&mut self, context: &mut EventContext) -> Result<(), Error>;
}
