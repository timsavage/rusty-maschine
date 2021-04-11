use crate::gui::display::{MonochromeCanvas, Canvas, Pixel};
use crate::gui::ui::{Control, EventHandler};
use crate::events::{Event, Direction};

///
/// Simple scrollable text panel
///
pub struct TextPanel {
    dirty: bool,
    text: String,
    scroll_pos: (usize, usize),
    text_size: (usize, usize),
    control_size: (usize, usize),
}

impl TextPanel {
    pub fn new() -> Self {
        TextPanel {
            dirty: false,
            text: String::from(""),
            scroll_pos: (0, 0),
            text_size: (0, 0),
            control_size: (0, 0),
        }
    }

    ///
    /// Set the current text
    ///
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);

        // Calculate size
        self.text_size = (
            self.text.chars().filter(|&c| c == '\n').count() + 1,
            self.text.lines().map(|x| x.len()).max().unwrap(),
        );

        self.dirty = true;
    }

    ///
    /// Current text
    ///
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

impl Control for TextPanel {
    fn set_size(&mut self, height: usize, width: usize) {
        self.control_size = (height, width);
        self.dirty = true;
    }

    fn set_repaint(&mut self) {
        self.dirty = true;
    }

    fn paint(&mut self, canvas: &mut MonochromeCanvas, row: usize, col: usize) {
        if self.dirty {
            let height = std::cmp::min(self.control_size.0, self.text_size.0);
            let v_scroll = self.scroll_pos.0;

            // Clear background
            canvas.fill_rows(row, row + self.control_size.0, Pixel::Off);

            for (idx, line) in self.text.lines().enumerate() {
                if idx < v_scroll { continue };
                if idx >= height + v_scroll { break };
                let current_row = row + idx - v_scroll;
                canvas.fill_row(current_row, Pixel::Off);
                canvas.print(line, current_row, col, Pixel::On);
            }

            self.dirty = false;
        }
    }
}

impl EventHandler for TextPanel {
    fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::EncoderChange(_encoder, direction, _shift) => {
                let v_scroll = self.scroll_pos.0;
                let text_height = self.text_size.0;
                let height = self.control_size.0;

                if text_height > height {
                    match direction {
                        Direction::Up => {
                            if v_scroll < (text_height - height) {
                                self.scroll_pos.0 += 1;
                                self.dirty = true;
                            }
                        },
                        Direction::Down => {
                            if v_scroll > 0 {
                                self.scroll_pos.0 -= 1;
                                self.dirty = true;
                            }
                        }
                    }
                }

                true
            },
            _ => { false }
        }
    }
}