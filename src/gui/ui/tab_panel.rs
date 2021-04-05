use crate::gui::display::MonochromeCanvas;
use crate::gui::ui::Control;

pub struct Tab {
    name: String,
}

pub struct TabPanel {
    tabs: [Option<Tab>; 3],
    current: usize,
}

impl TabPanel {
    fn new() -> Self {
        TabPanel {
            tabs: [None, None, None],
            current: 0,
        }
    }
}
