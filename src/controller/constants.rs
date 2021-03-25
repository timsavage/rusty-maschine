///
/// Controller events
///
#[derive(Debug)]
#[allow(dead_code)]
pub enum Event {
    ///
    /// Button change (Button, Pressed, Shift)
    ///
    ButtonChange(Button, bool, bool),

    ///
    /// Encoder change (Encoder Number, Direction, Shift)
    ///
    EncoderChange(u8, Direction, bool),

    ///
    /// Pad change (Pad Number, Velocity, Shift)
    ///
    PadChange(u8, u8, bool),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Button {
    Erase,
    Rec,
    Play,
    Grid,
    TransportRight,
    TransportLeft,
    Restart,
    MainEncoder,
    NoteRepeat,
    Sampling,
    Browse,
    Group,
    Main,
    BrowseRight,
    BrowseLeft,
    Nav,
    Control,
    F3,
    F2,
    F1,
    Mute,
    Solo,
    Select,
    Duplicate,
    View,
    PadMode,
    Pattern,
    Scene,
    Unknown,
}

///
/// Direction of encoder
///
#[derive(Debug)]
#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
}
