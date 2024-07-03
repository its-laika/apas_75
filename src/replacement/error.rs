use std::fmt::{Display, Formatter, Result};

#[allow(clippy::module_name_repetitions)]
pub enum ReplacementError {
    NotReadable(String),
    NotWriteable(String),
    InvalidConfig,
}

impl Display for ReplacementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::InvalidConfig => write!(
                f,
                "Starship configuration does not contain valid theme data"
            ),
            Self::NotReadable(path) => write!(f, "File {path} is not readeable"),
            Self::NotWriteable(path) => write!(f, "File {path} is not writeable"),
        }
    }
}
