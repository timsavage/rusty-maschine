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
