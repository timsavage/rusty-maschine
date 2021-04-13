use crate::events::{Button, Event};
use crate::gui::display::{Canvas, MonochromeCanvas, Pixel};
use crate::gui::ui::{Control, EventHandler};

///
/// Individual tab definition
///
pub struct Tab {
    name: String,
    control: Box<dyn Control>,
}

impl Tab {
    fn paint(&self, canvas: &mut MonochromeCanvas, row: usize, col: usize, current: bool) {
        // Render tab bar
        canvas.set_pixel(col, 1, Pixel::On);
        canvas.set_pixel(col, 2, Pixel::On);
        canvas.set_pixel(col, 3, Pixel::On);
        canvas.set_pixel(col, 4, Pixel::On);
        canvas.set_pixel(col, 5, Pixel::On);
        canvas.set_pixel(col, 6, Pixel::On);
        canvas.set_pixel(col, 7, Pixel::On);
        canvas.set_pixel(col + 41, 1, Pixel::On);
        canvas.set_pixel(col + 41, 2, Pixel::On);
        canvas.set_pixel(col + 41, 3, Pixel::On);
        canvas.set_pixel(col + 41, 4, Pixel::On);
        canvas.set_pixel(col + 41, 5, Pixel::On);
        canvas.set_pixel(col + 41, 6, Pixel::On);
        canvas.set_pixel(col + 41, 7, Pixel::On);

        canvas.print(self.name.as_str(), row, col + 2, Pixel::On);
        if current {
            canvas.invert_row_slice(row, col + 1, col + 41);
        } else {
            for idx in 1..41 {
                canvas.set_pixel(col + idx, 0, Pixel::On);
            }
        }
    }
}

///
/// Full screen container that includes 3 sub panels
///
pub struct TabPanel {
    dirty: bool,
    tabs: [Option<Tab>; 3],
    current: usize,
    size: (usize, usize),
}

impl TabPanel {
    pub fn new() -> Self {
        TabPanel {
            dirty: false,
            tabs: [None, None, None],
            current: 0,
            size: (0, 0),
        }
    }

    pub fn add_tab(&mut self, idx: usize, name: &str, control: Box<dyn Control>) {
        self.tabs[idx] = Some(Tab {
            name: String::from(name),
            control,
        });
        self.dirty = true;
    }

    fn _set_tab(&mut self, idx: usize) {
        self.current = idx;
        self.dirty = true;

        let tab = &mut self.tabs[idx];
        if tab.is_some() {
            let tab: &mut Tab = &mut tab.as_mut().unwrap();
            tab.control.set_repaint();
        }
    }
}

impl EventHandler for TabPanel {
    fn handle(&mut self, event: &Event) -> bool {
        let mut handled = false;

        match event {
            Event::ButtonChange(button, pressed, _shift) => match button {
                Button::F1 => {
                    if *pressed {
                        self._set_tab(0);
                    }
                }
                Button::F2 => {
                    if *pressed {
                        self._set_tab(1);
                    }
                }
                Button::F3 => {
                    if *pressed {
                        self._set_tab(2);
                    }
                }
                _ => {}
            },
            _ => {}
        }

        // Pass on event
        let tab = &mut self.tabs[self.current];
        if tab.is_some() {
            let tab: &mut Tab = &mut tab.as_mut().unwrap();
            handled = tab.control.handle(event);
        }

        handled
    }
}

impl Control for TabPanel {
    fn set_size(&mut self, height: usize, width: usize) {
        self.size = (height, width);
        for (_, tab) in self.tabs.iter_mut().enumerate() {
            if tab.is_some() {
                let tab: &mut Tab = &mut tab.as_mut().unwrap();
                tab.control.set_size(height - 1, width);
            }
        }
    }

    /// Force a repaint
    fn set_repaint(&mut self) {
        self.dirty = true;
    }

    fn paint(&mut self, canvas: &mut MonochromeCanvas, row: usize, col: usize) {
        // Render tab bar if required
        if self.dirty {
            // Clear the row
            canvas.fill_row(row, Pixel::Off);

            // Paint each tab
            for (idx, tab) in self.tabs.iter_mut().enumerate() {
                if tab.is_some() {
                    let tab: &mut Tab = &mut tab.as_mut().unwrap();
                    tab.paint(canvas, row, col + (idx * 42) + idx, idx == self.current);
                }
            }

            // Clear dirty flag
            self.dirty = false;
        }

        // Render the current control
        self.tabs[self.current]
            .as_mut()
            .map(|t| t.control.paint(canvas, row + 1, col));
    }
}
