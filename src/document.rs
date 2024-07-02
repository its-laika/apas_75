use std::fmt::{Debug, Display, Formatter};

const NEW_LINE: char = '\n';
const SEPARATOR_THEME_START: &str = "### apas_75 theme start ###";
const SEPARATOR_THEME_END: &str = "### apas_75 theme end ###";

pub enum ConfigDocumentError {
    MissingThemeIndicator,
}

impl Debug for ConfigDocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingThemeIndicator => {
                write!(f, "Not exactly one start and one end theme indicator given")
            }
        }
    }
}

pub struct ConfigDocument {
    pub default: String,
    pub theme: Option<String>,
}

impl ConfigDocument {
    pub fn new(file_content: &str) -> Result<ConfigDocument, ConfigDocumentError> {
        match (
            get_single_index(file_content, SEPARATOR_THEME_START),
            get_single_index(file_content, SEPARATOR_THEME_END),
        ) {
            (Some(theme_start), Some(theme_end)) => {
                let default = format!(
                    "{}{}{}",
                    &file_content[0..theme_start],
                    NEW_LINE,
                    &file_content[(theme_end + SEPARATOR_THEME_END.len())..]
                );

                let theme = Some(String::from(
                    &file_content[(theme_start + SEPARATOR_THEME_START.len())..theme_end],
                ));

                Ok(ConfigDocument { default, theme })
            }
            (None, None) => Ok(ConfigDocument {
                default: String::from(file_content),
                theme: None,
            }),
            _ => Err(ConfigDocumentError::MissingThemeIndicator),
        }
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.theme = Some(String::from(theme));
    }
}

impl Display for ConfigDocument {
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
