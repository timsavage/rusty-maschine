use hidapi::HidError;
use rand::Rng;


///
/// Common controller errors
/// 
#[derive(Debug)]
pub enum Error {
    HidAPI(HidError),

    /// Input buffer does not container the expected amount of data.
    BufferUnderrun,

    /// Unexpected control returned from hardware device
    UnknownControl,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::HidAPI(e)      => e.fmt(fmt),  // Pass on to HIDAPI interface
            Error::BufferUnderrun => write!(fmt, "Buffer does not contain the expected amount of data"),
            Error::UnknownControl => write!(fmt, "Unexpected control returned from hardware device"),
        }        
    }
}

impl From<HidError> for Error {
    fn from(err: HidError) -> Error {
        Error::HidAPI(err)
    }
}


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
    pub fn red() -> Self {
        Colour::new(255, 0, 0)
    }
    pub fn green() -> Self {
        Colour::new(0, 255, 0)
    }
    pub fn blue() -> Self {
        Colour::new(0, 0, 255)
    }
    pub fn white() -> Self {
        Colour::new(255, 255, 255)
    }
    pub fn black() -> Self {
        Colour::new(0, 0, 0)
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


///
/// Common controller behaviours
/// 
pub trait Controller {
    /// Perform any update events with the contoller device
    fn tick(&mut self) -> Result<(), Error>;

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) -> Result<(), Error>;

    /// Explicity turn an LED off (black)
    fn set_led_off(&mut self, led: u8) -> Result<(), Error> {
        self.set_led(led, Colour::black())
    }

    /// The specified LED is an RGB led
    fn is_rgb_led(&self, led: u8) -> bool;
}
