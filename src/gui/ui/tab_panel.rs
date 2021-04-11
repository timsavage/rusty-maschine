use crate::events::{Button, Event};
use crate::gui::display::{Canvas, MonochromeCanvas, Pixel};
use crate::gui::ui::{Control, EventHandler};

pub struct Tab {
    name: String,
    control: Box<dyn Control>,
}

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

        let mut tab = &mut self.tabs[idx];
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
                    if *pressed { self._set_tab(0); }
                }
                Button::F2 => {
                    if *pressed { self._set_tab(1); }
                }
                Button::F3 => {
                    if *pressed { self._set_tab(2); }
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
        for (idx, tab) in self.tabs.iter_mut().enumerate() {
            if tab.is_some() {
                let tab: &mut Tab = &mut tab.as_mut().unwrap();
                 tab.control.set_size(height - 1, width);
            }
        }
    }

    fn set_repaint(&mut self) {
        self.dirty = true;
    }

    fn paint(&mut self, canvas: &mut MonochromeCanvas, row: usize, col: usize) {
        if self.dirty {
            // Clear the row
            canvas.fill_row(row, Pixel::Off);
        }

        for (idx, tab) in self.tabs.iter_mut().enumerate() {
            if tab.is_some() {
                let tab: &mut Tab = &mut tab.as_mut().unwrap();

                // Render tab bar
                if self.dirty {
                    let start = col + (idx * 42) + idx;
                    let end = start + 42;
                    canvas.print(
                        tab.name.as_str(),
                        row,
                        start + 1,
                        Pixel::On,
                    );
                    if idx != self.current {
                        canvas.invert_row_slice(row, start, end);
                    }
                }

                // Render panel
                if idx == self.current {
                    tab.control.paint(canvas, row + 1, col);
                }
            }
        }

        self.dirty = false;
    }
}
