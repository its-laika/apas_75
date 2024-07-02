use std::fmt::{Debug, Formatter, Result};

#[allow(clippy::module_name_repetitions)]
/// Contains all `Error` cases that the `Config` uses
pub enum ConfigError {
    /// Indicates that a theme part cannot be unambiguously found
    MissingThemeIndicator,
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MissingThemeIndicator => {
                write!(f, "Not exactly one start and one end theme indicator given")
            }
        }
    }
}
