///
/// Colour definition
///
/// Can represent RGB or Mono colours
///
#[derive(Copy, Clone, Default)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub const BLACK: Colour = Colour { r: 0, g: 0, b: 0 };
    pub const WHITE: Colour = Colour {
        r: 255,
        g: 255,
        b: 255,
    };
    #[allow(dead_code)]
    pub const RED: Colour = Colour { r: 255, g: 0, b: 0 };
    #[allow(dead_code)]
    pub const GREEN: Colour = Colour { r: 0, g: 255, b: 0 };
    #[allow(dead_code)]
    pub const BLUE: Colour = Colour { r: 0, g: 0, b: 255 };

    /// Construct a new colour
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b }
    }

    /// "Monochrome" representation of the colour
    pub fn as_1bit(&self) -> u8 {
        if (self.r > 0x7F) | (self.g > 0x7F) | (self.b > 0x7F) {
            0xFF
        } else {
            0x00
        }
    }

    /// Return the components of this colour
    pub fn components(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}
