use hidapi::HidError;


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
