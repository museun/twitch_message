#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    MalformedMessage,
    InvalidNumeric,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedMessage => f.write_str("the message was malformed"),
            Self::InvalidNumeric => f.write_str("invalid message numeric"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
