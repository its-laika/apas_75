use std::fmt::{Display, Formatter};

use crate::join_trimmed;

use super::error::ConfigError;

const NEW_LINE: &str = "\n";
const SEPARATOR_THEME_START: &str = "### apas_75 theme start ###";
const SEPARATOR_THEME_END: &str = "### apas_75 theme end ###";

/// Representation of a TOML config file that may contains a custom theme.
pub struct Config {
    default: String,
    theme: Option<String>,
}

impl Config {
    /// Tries building a `Config` based on content of a TOML file
    ///
    /// # Arguments
    ///
    /// * `file_content` - Content of TOML file
    ///
    /// # Returns
    ///
    /// Config or `Err` if the file cannot be split into default config and
    /// theme part correctly. May happen if file is corrupted.
    pub fn new(file_content: &str) -> Result<Config, ConfigError> {
        match (
            get_single_index(file_content, SEPARATOR_THEME_START),
            get_single_index(file_content, SEPARATOR_THEME_END),
        ) {
            (Some(theme_start), Some(theme_end)) => {
                let default = join_trimmed!(
                    NEW_LINE,
                    &file_content[0..theme_start],
                    &file_content[(theme_end + SEPARATOR_THEME_END.len())..]
                );

                let theme = Some(join_trimmed!(
                    NEW_LINE,
                    &file_content[(theme_start + SEPARATOR_THEME_START.len())..theme_end]
                ));

                Ok(Config { default, theme })
            }
            (None, None) => Ok(Config {
                default: String::from(file_content.trim()),
                theme: None,
            }),
            _ => Err(ConfigError::MissingThemeIndicator),
        }
    }

    /// Overrides `theme` part of `Config`
    ///
    /// # Arguments
    ///
    /// * `theme` - Theme data
    pub fn set_theme(&mut self, theme: &str) {
        self.theme = Some(String::from(theme));
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.theme {
            Some(theme) => {
                write!(
                    f,
                    "{}{}{}{}{}{}{}{}{}",
                    self.default.trim(),
                    NEW_LINE,
                    NEW_LINE,
                    SEPARATOR_THEME_START,
                    NEW_LINE,
                    theme.trim(),
                    NEW_LINE,
                    SEPARATOR_THEME_END,
                    NEW_LINE
                )
            }
            None => write!(f, "{}", self.default.trim()),
        }
    }
}

/// Tries getting the **only** index of a `needle` in a `haystack`.
/// If `haystack` contains `needle` not exactly **one** time, `None` is returned.
///
/// # Arguments
///
/// * `haystack` - String to search for `needle`
/// * `needle` - The string that should be searched for in `haystack`
///
/// # Returns
///
/// Index of the **only** substring in `haystack` or `None`.
///
/// # Examples
///
/// ```
/// assert_eq!(get_single_index("Starships were meant to fly", "a"), None); // because "a" exists more than once
/// assert_eq!(get_single_index("Starships were meant to fly", "were"), Some(10));
/// ```
fn get_single_index(haystack: &str, needle: &str) -> Option<usize> {
    let matches = haystack
        .match_indices(needle)
        .take(2)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_works_on_start() {
        let toml = "### apas_75 theme start ###\ncustom\nlines\n### apas_75 theme end ###\n\ndefault\nlines";
        let config_result = Config::new(toml);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.default, "default\nlines");
        assert!(config.theme.is_some());
        assert_eq!(config.theme.unwrap(), "custom\nlines");
    }

    #[test]
    fn test_split_works_in_middle() {
        let toml = "default\nlines\nstart\n\n### apas_75 theme start ###\ncustom\nlines\n### apas_75 theme end ###\ndefault\nlines\nend";
        let config_result = Config::new(toml);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.default, "default\nlines\nstart\ndefault\nlines\nend");
        assert!(config.theme.is_some());
        assert_eq!(config.theme.unwrap(), "custom\nlines");
    }

    #[test]
    fn test_split_works_on_end() {
        let toml = "default\nlines\nstart\ndefault\nlines\nend\n\n### apas_75 theme start ###\ncustom\nlines\n### apas_75 theme end ###";
        let config_result = Config::new(toml);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.default, "default\nlines\nstart\ndefault\nlines\nend");
        assert!(config.theme.is_some());
        assert_eq!(config.theme.unwrap(), "custom\nlines");
    }

    #[test]
    fn test_split_works_without_theme() {
        let toml = "default\nlines\nstart\ndefault\nlines\nend";
        let config_result = Config::new(toml);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.default, "default\nlines\nstart\ndefault\nlines\nend");
        assert!(config.theme.is_none());
    }

    #[test]
    fn test_appends_works() {
        let toml = "default\nlines\nstart\ndefault\nlines\nend";
        let config_result = Config::new(toml);
        assert!(config_result.is_ok());

        let mut config = config_result.unwrap();
        assert_eq!(config.default, "default\nlines\nstart\ndefault\nlines\nend");
        assert!(config.theme.is_none());

        config.set_theme("custom\nlines");

        let result = config.to_string();

        assert_eq!(
            result,
            "default\nlines\nstart\ndefault\nlines\nend\n\n### apas_75 theme start ###\ncustom\nlines\n### apas_75 theme end ###\n"
        );
    }
}
