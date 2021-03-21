use rand::Rng;

pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Colour {
            red: red,
            green: green,
            blue: blue,
        }
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
    pub fn on() -> Self {
        Colour::new(255, 255, 255)
    }
    pub fn off() -> Self {
        Colour::new(0, 0, 0)
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Colour::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>())
    }

    pub fn as_mono(&self) -> u8 {
        if (self.red > 128) | (self.green > 128) | (self.blue > 128) {
            255
        } else {
            0
        }
    }

    /// Return the components of this colour
    pub fn components(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

pub trait Controller {
    /// Perform any update events with the contoller device
    fn tick(&mut self) -> Result<(), ()>;

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) -> Result<(), ()>;

    /// Explicity turn an LED off
    fn set_led_off(&mut self, led: u8) -> Result<(), ()> {
        self.set_led(led, Colour::off())
    }

    /// The specified LED is an RGB led
    fn is_rgb_led(&self, led: u8) -> bool;
}
