///
/// Display interface
///
use super::font::{FONT_5X6, FONT_5X6_WIDTH};

///
/// State of a pixel
///
#[derive(Clone)]
pub enum Pixel {
    On,
    Off,
}

///
/// Basic display interface
///
pub trait Canvas<T> {
    ///
    /// Width of the display
    ///
    fn width(&self) -> usize;

    ///
    /// Height of the display
    ///
    fn height(&self) -> usize;

    ///
    /// Get display data
    ///
    fn data_size(&self) -> usize;

    ///
    /// Get display data
    ///
    fn data(&self) -> &[u8];

    ///
    /// Data is dirty (has changed since last clear)
    ///
    fn is_dirty(&self) -> bool;

    ///
    /// Clear the data dirty flag
    ///
    fn clear_dirty_flag(&mut self);

    ///
    /// Invert all pixels
    ///
    fn invert(&mut self);

    ///
    /// Invert a row (8 pixels)
    ///
    fn invert_row(&mut self, row: usize);

    ///
    /// Fill the entire canvas with a single colour
    ///
    fn fill(&mut self, colour: T);

    ///
    /// Set a pixel
    ///
    fn set_pixel(&mut self, x: usize, y: usize, colour: T);

    ///
    /// Get the state of a pixel
    ///
    fn pixel(&self, x: usize, y: usize) -> Option<T>;

    ///
    /// Copy canvas
    ///
    fn copy_from(&mut self, canvas: &dyn Canvas<T>);

    ///
    /// Print
    ///
    fn print(&mut self, text: &str, row: usize, col: usize, colour: Pixel);

    ///
    /// Print character
    ///
    fn print_char(&mut self, t: char, row: usize, col: usize, colour: Pixel);
}

///
/// Monochrome display that uses 1bpp for data display.
///
/// Optimally the display width is a multiple of 8.
///
pub struct MonochromeCanvas {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    dirty: bool,
}

impl MonochromeCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        MonochromeCanvas {
            width,
            height,
            buffer: vec![0; (width * height) / 8],
            dirty: true,
        }
    }

    pub fn from_buffer(width: usize, height: usize, buffer: &[u8]) -> Self {
        let buffer_size = (width * height) / 8;
        if buffer.len() != buffer_size {
            panic!("Buffer must be {} bytes long", buffer_size)
        }

        MonochromeCanvas {
            width,
            height,
            buffer: buffer.to_vec(),
            dirty: true,
        }
    }
}

impl Canvas<Pixel> for MonochromeCanvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data_size(&self) -> usize {
        self.buffer.len()
    }

    fn data(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn clear_dirty_flag(&mut self) {
        self.dirty = false;
    }

    fn invert(&mut self) {
        for byte in self.buffer.iter_mut() {
            *byte = !(*byte);
        }
        self.dirty = true;
    }

    ///
    /// Invert a row (8 pixels)
    ///
    fn invert_row(&mut self, row: usize) {
        let offset = row << 7; // Multiply by 128
        if offset < self.data_size() {
            for idx in offset..(offset + self.width) {
                self.buffer[idx] = !self.buffer[idx];
            }
        }
    }

    ///
    /// Fill the entire display with a Pixel
    ///
    fn fill(&mut self, colour: Pixel) {
        let value = match colour {
            Pixel::On => 0xFFu8,
            Pixel::Off => 0x00u8,
        };

        for byte in self.buffer.iter_mut() {
            *byte = value;
        }

        self.dirty = true;
    }

    ///
    /// Set a pixel
    ///
    fn set_pixel(&mut self, x: usize, y: usize, colour: Pixel) {
        let width = self.width();
        let height = self.height();
        if (x > width) | (y > height) {
            return;
        }

        let byte_index = (width * (y >> 3)) + x;
        match colour {
            Pixel::On => self.buffer[byte_index] |= 1 << (y & 7),
            Pixel::Off => self.buffer[byte_index] &= !(1 << (y & 7)),
        }

        self.dirty = true;
    }

    ///
    /// Get state of a pixel
    ///
    fn pixel(&self, x: usize, y: usize) -> Option<Pixel> {
        if (x > self.width) | (y > self.height) {
            return None;
        }

        let byte_index = (self.width * (y >> 3)) + x;
        let pixel = self.buffer[byte_index] >> ((y & 7) & 0x01);
        Some(if pixel == 0 { Pixel::Off } else { Pixel::On })
    }

    ///
    /// Copy canvas
    ///
    fn copy_from(&mut self, canvas: &dyn Canvas<Pixel>) {
        self.buffer = canvas.data().to_vec();
    }

    ///
    /// Print
    ///
    fn print(&mut self, s: &str, row: usize, col: usize, colour: Pixel) {
        let bytes = s.as_bytes();
        for idx in 0..s.len() {
            self.print_char(bytes[idx] as char, row, col + (idx * 6), colour.clone())
        }
    }

    ///
    /// Print single character
    ///
    fn print_char(&mut self, c: char, row: usize, col: usize, colour: Pixel) {
        let raw = c as usize;
        if raw < 0x20 || raw > 0x7F {
            return;
        }
        let char_idx = (raw - 0x20) * FONT_5X6_WIDTH;
        for slice in 0..FONT_5X6_WIDTH {
            self.buffer[(row * self.width) + col + slice] = match colour {
                Pixel::On => FONT_5X6[char_idx + slice],
                Pixel::Off => !FONT_5X6[char_idx + slice],
            }
        }
        self.dirty = true;
    }
}
