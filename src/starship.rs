use crate::FILE_EXTENSION_TOML;
use std::{io::Error, path::PathBuf};

/// Calculates next Starship theme file to use based on given theme `config_paths` and
/// currently used `current_config_path`.
///
/// # Arguments
///
/// * `config_paths` - All valid Starship theme config paths
/// * `current_config_path` - The currently used config path, based on the Starship
///   config environment variable.
///
/// # Returns
///
/// Either `PathBuf` of the next Starship config to use or `Err` if no next theme can
/// be determined.
///
/// # Examples
///
/// * see [main.rs](./main.rs)
pub fn get_next_theme_file_name(
    config_paths: &[PathBuf],
    current_config_path: Option<String>,
) -> Result<PathBuf, Error> {
    let mut theme_files = config_paths
        .iter()
        .filter(|f| f.extension().is_some_and(|e| e == FILE_EXTENSION_TOML))
        .collect::<Vec<&PathBuf>>();

    theme_files.sort();

    let first = match theme_files.first() {
        Some(&f) => f.clone(),
        None => {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "No theme files available",
            ))
        }
    };

    if config_paths.len() == 1 {
        return Ok(first);
    }

    let Some(current_config_path) = current_config_path else {
        return Ok(first);
    };

    let next = theme_files.iter().find(|&f| match f.to_str() {
        Some(s) => s > &current_config_path,
        None => false,
    });

    match next {
        Some(&s) => Ok(s.clone()),
        None => Ok(first),
    }
}
