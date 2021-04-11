struct Control {
    height: usize,
    width: usize,
}

struct Display {
    control: Control,
}

impl Display {
    pub fn control(&self) -> &Control {
        &self.control
    }
}
