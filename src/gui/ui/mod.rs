mod display;
mod list_panel;
mod tab_panel;
mod text_panel;

use crate::events::{Event, EventHandler};
use crate::gui::display::MonochromeCanvas;
pub use list_panel::ListPanel;
pub use tab_panel::TabPanel;
pub use text_panel::TextPanel;

/// Basic control trait
pub trait Control: EventHandler {
    /// Set the height and width of the control
    fn set_size(&mut self, height: usize, width: usize);

    /// Tell control it needs to repaint.
    ///
    /// This occurs when a control is made visible eg in a tab panel
    fn set_repaint(&mut self);

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
            child: None,
        }
    }

    /// Paint the service
    pub fn paint(&mut self, canvas: &mut MonochromeCanvas) {
        if self.child.is_some() {
            self.child.as_mut().unwrap().paint(canvas, 0, 0);
        }
    }

    /// Add a the current child
    pub fn set_child(&mut self, child: T) {
        self.child = Some(child);
        self.child
            .as_mut()
            .unwrap()
            .set_size(self.size.0, self.size.1)
    }

    /// Handle events
    pub fn handle(&mut self, event: &Event) -> bool {
        return self.child.as_mut().map_or(false, |c| c.handle(&event));
    }
}
