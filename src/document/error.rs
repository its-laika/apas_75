use std::fmt::{Debug, Formatter, Result};

/// Contains all `Error` cases that the `Config` uses
#[allow(clippy::module_name_repetitions)]
pub enum ConfigError {
    /// Indicates that a theme part cannot be unambiguously found
    MissingThemeIndicator,
    /// Indicates that theme indicators have the wrong order
    ThemeIndicatorsWrongOrder,
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MissingThemeIndicator => {
                write!(f, "Not exactly one start and one end theme indicator given")
            }
            Self::ThemeIndicatorsWrongOrder => {
                write!(f, "Theme indicators not in the correct order (start > end)")
            }
        }
    }
}
