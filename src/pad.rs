use crate::colour::Colour;
use crate::controller::{Controller, Error};
use crate::events::Event::ButtonChange;
use crate::events::{Button, Event, EventContext, EventHandler, EventTask};
use rand::Rng;
use std::time::Instant;

pub struct Light {
    x: usize,
    y: usize,
    v: usize,
    colour: Colour,
}

pub struct Rainbow {
    active: bool,
    dirty: bool,
    clock: Instant,
    next_frame: u128,
    lights: Vec<Light>,
}

impl Rainbow {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut lights: Vec<Light> = Vec::new();
        for _ in 0..8 {
            lights.push(Light {
                x: rng.gen_range(0..4),
                y: rng.gen_range(0..4),
                v: 1,
                colour: Colour::BLACK,
            });
        }

        Self {
            active: false,
            dirty: false,
            clock: Instant::now(),
            next_frame: 0,
            lights,
        }
    }

    pub fn render(&mut self, ctlr: &mut dyn Controller) {
        if self.active {
            let ms = self.clock.elapsed().as_millis();
            if ms >= self.next_frame {
                let mut rng = rand::thread_rng();

                self.next_frame = ms + 500;

                // Render lights
                for pad in 0..16 {
                    ctlr.set_pad_led(pad as u8, Colour::BLACK);
                }
                for light in self.lights.iter() {
                    let pad = light.x + (light.y * 4);
                    ctlr.set_pad_led(pad as u8, light.colour);
                }

                // Move lights
                for light in self.lights.iter_mut() {
                    light.y += light.v;
                    if light.y >= 4 {
                        light.y = 0;
                        light.x = rng.gen_range(0..4);
                        light.colour = Colour::random();
                    }
                }
            }
        }

        if self.dirty {
            ctlr.set_button_led(
                Button::Play,
                if self.active {
                    Colour::WHITE
                } else {
                    Colour::BLACK
                },
            );
            self.dirty = false;
        }
    }
}

impl EventHandler for Rainbow {
    fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::ButtonChange(btn, pressed, _) => match btn {
                Button::Play => {
                    if *pressed {
                        self.active = !self.active;
                        self.dirty = true;
                        return true;
                    }
                }
                _ => {}
            },
            // Event::PadChange(pad, velocity, shift) => true,
            _ => {}
        };
        false
    }
}
