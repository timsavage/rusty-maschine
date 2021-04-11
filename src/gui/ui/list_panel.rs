use crate::events::{Button, Direction, Event};
use crate::gui::display::{Canvas, MonochromeCanvas, Pixel};
use crate::gui::ui::{Control, EventHandler};

pub struct ListPanel<T> {
    dirty: bool,
    list: Vec<(T, String)>,
    current: usize,
    offset: usize,
    control_size: (usize, usize),
}

impl<T> ListPanel<T> {
    pub fn new() -> Self {
        ListPanel {
            dirty: false,
            list: Vec::new(),
            current: 0,
            offset: 0,
            control_size: (0, 0),
        }
    }

    pub fn add_item(&mut self, key: T, text: &str) {
        self.list.push((key, String::from(text)));
    }

    pub fn set_current(&mut self, idx: usize) {
        let count = self.list.len();
        if count == 0 || idx >= count {
            self.current = 0;
            self.offset = 0;
        } else {
            self.current = idx;
            let height = self.control_size.0;
            self.offset = if count > height {
                ((count - height + 1) * idx) / count
            } else {
                0
            };
        };
        self.dirty = true;
    }
}

impl<T> EventHandler for ListPanel<T> {
    fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::EncoderChange(_, direction, shift) => match direction {
                Direction::Up => {
                    self.set_current(if self.current == 0 {
                        self.list.len() - 1
                    } else {
                        self.current - 1
                    });
                }
                Direction::Down => {
                    self.set_current(self.current + 1);
                }
            },
            _ => {}
        }
        false
    }
}

impl<T> Control for ListPanel<T> {
    fn set_size(&mut self, height: usize, width: usize) {
        self.control_size = (height, width);
    }

    fn set_repaint(&mut self) {
        self.dirty = true;
    }

    fn paint(&mut self, canvas: &mut MonochromeCanvas, row: usize, col: usize) {
        if self.dirty {
            // Clear background
            canvas.fill_rows(row, row + self.control_size.0, Pixel::Off);

            let start = self.offset;
            let end = std::cmp::min(self.control_size.0 + self.offset, self.list.len());

            for (idx, (key, text)) in self.list[start..end].iter().enumerate() {
                let idx = idx;
                canvas.print(text.as_str(), idx + row, col + 1, Pixel::On);
                if (idx + self.offset) == self.current {
                    canvas.invert_row(idx + row);
                }
            }

            self.dirty = false;
        }
    }
}
