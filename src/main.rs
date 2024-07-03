use std::process::ExitCode;

use files::{build_full_path, DIRECTORY_SEPARATOR};
use replacement::replace_custom_theme;

mod document;
mod files;
mod random;
mod replacement;
mod strings;

const CONFIG_DIR: &str = "~/.config/";
const STARSHIP_FILE_NAME: &str = "starship.toml";
const THEME_FILE_PREFIX: &str = "starship-theme-";

fn main() -> ExitCode {
    let config_dir_path = match build_full_path(CONFIG_DIR) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not build full path to Starship config: {e}");
            return ExitCode::FAILURE;
        }
    };

    let starship_config_path = format!(
        "{}{}{}",
        &config_dir_path, DIRECTORY_SEPARATOR, STARSHIP_FILE_NAME
    );

    if let Err(e) = replace_custom_theme(&starship_config_path, &config_dir_path, THEME_FILE_PREFIX)
    {
        eprintln!("apas_75 failed with error: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
