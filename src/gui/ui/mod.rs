mod tab_panel;
mod text_panel;

use crate::gui::display::MonochromeCanvas;
// pub use tab_panel::{TabPanel};
pub use text_panel::{TextPanel};
use crate::events::Event;

pub trait EventHandler {

    /// Handle event and return if it was handled.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to the event to be handled
    ///
    fn handle(&mut self, event: &Event) -> bool;
}

/// Basic control trait
pub trait Control: EventHandler {

    /// Set the height and width of the control
    fn set_size(&mut self, height: usize, width: usize);

    /// Paint the control to the display if necessary
    ///
    /// # Arguments
    ///
    /// * `canvas` - The current drawing canvas
    /// * `row` - Row offset to start drawing
    /// * `col` - Column offset to start drawing
    ///
    fn paint(&mut self, canvas: &mut MonochromeCanvas, row: usize, col: usize);
}

pub struct Surface<T: Control> {
    size: (usize, usize),
    child: Option<T>,
}

impl<T: Control> Surface<T> {
    pub fn new(height: usize, width: usize) -> Self {
        Surface {
            size: (height / 8, width),
            child: None
        }
    }

    pub fn paint(&mut self, canvas: &mut MonochromeCanvas) {
        if self.child.is_some() {
            self.child.as_mut().unwrap().paint(canvas, 0, 0);
        }
    }

    pub fn set_child(&mut self, child: T) {
        self.child = Some(child);
        self.child.as_mut().unwrap().set_size(self.size.0, self.size.1)
    }

    pub fn handle(&mut self, event: &Event) {
        if self.child.is_some() {
            self.child.as_mut().unwrap().handle(&event);
        }
    }
}