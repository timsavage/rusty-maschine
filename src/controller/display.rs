///
/// Display interface
/// 


///
/// Basic display interface
/// 
pub trait Display {
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
    /// Fill all pixels
    /// 
    fn fill(&mut self, state: Pixel);
}


pub enum Pixel {
    On,
    Off,
}


///
/// Monochome display that uses 1bpp for data display.
/// 
/// Optimally the display width is a multiple of 8.
/// 
pub struct MonochromeDisplay {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    dirty: bool,
}

impl Display for MonochromeDisplay {
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

    fn fill(&mut self, state: Pixel) {
        let value = match state {
            Pixel::On => 1u8,
            Pixel::Off => 0u8,
        };

        for byte in self.buffer.iter_mut() {
            *byte = value;
        }

        self.dirty = true;
    }
}

impl MonochromeDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        MonochromeDisplay {
            width, 
            height,
            buffer: vec![0; (width * height) / 8],
            dirty: true
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, state: Pixel) {
        let width = self.width();
        let height = self.height();

        if (x > width) | (y > height) {
            return;
        }

        let byte_index = (width * (y >> 3)) + x;

        match state {
            Pixel::On => self.buffer[byte_index] |= 1 << (y & 7),
            Pixel::Off => self.buffer[byte_index] &= !(1 << (y & 7)),
        }

        self.dirty = true;
    }

    fn pixel(&self, x: usize, y: usize) -> Option<Pixel> {
        if (x > self.width) | (y > self.height) {
            return None;
        }
        
        let byte_index = (self.width * (y >> 3)) + x;
        let pixel = self.buffer[byte_index] >> ((y & 7) & 0x01);
        Some(if pixel == 0 { Pixel::Off } else { Pixel::On })
    }
}
