use rand::Rng;

///
/// Colour definition
///
/// Can represent RGB or Mono colours
///
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Colour { red, green, blue }
    }

    /// A random colour
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Colour::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>())
    }

    /// "Monochome" representation of the colour
    pub fn as_mono(&self) -> u8 {
        if (self.red > 0x7F) | (self.green > 0x7F) | (self.blue > 0x7F) {
            0xFF
        } else {
            0x00
        }
    }

    /// Return the components of this colour
    pub fn components(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

pub const BLACK: Colour = Colour {
    red: 0x00,
    green: 0x00,
    blue: 0x00,
};
pub const WHITE: Colour = Colour {
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};
pub const RED: Colour = Colour {
    red: 0xFF,
    green: 0x00,
    blue: 0x00,
};
pub const GREEN: Colour = Colour {
    red: 0x00,
    green: 0xFF,
    blue: 0x00,
};
pub const BLUE: Colour = Colour {
    red: 0x00,
    green: 0x00,
    blue: 0xFF,
};
