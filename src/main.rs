use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use document::Config;
use files::{build_full_path, get_file_names_of_directory, DIRECTORY_SEPARATOR};

mod document;
mod files;
mod random;

const STARSHIP_CONFIG_DIR: &str = "~/.config/";
const STARSHIP_FILE_NAME: &str = "starship.toml";

const PREFIX_THEMES_TOML: &str = "theme-";
const SUFFIX_THEMES_TOML: &str = ".toml";

fn main() {
    let config_dir_path = match build_full_path(STARSHIP_CONFIG_DIR) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not build full path to Starship config: {e}");
            return;
        }
    };

    let Some(theme_path) = get_random_theme_path(&config_dir_path) else {
        return;
    };

    let starship_config_path = format!(
        "{}{}{}",
        &config_dir_path, DIRECTORY_SEPARATOR, STARSHIP_FILE_NAME
    );

    let mut theme_config_file = match File::open(&theme_path) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Could not open theme file {theme_path}: {e}");
            return;
        }
    };

    let mut theme_config = String::new();
    if let Err(e) = theme_config_file.read_to_string(&mut theme_config) {
        eprint!("Could not read theme config file {theme_path}: {e}");
        return;
    }

    let mut starship_config = String::new();
    {
        let mut starship_config_file = match File::open(&starship_config_path) {
            Ok(f) => f,
            Err(e) => {
                eprint!("Could not open Starship config file {starship_config_path}: {e}");
                return;
            }
        };

        if let Err(e) = starship_config_file.read_to_string(&mut starship_config) {
            eprint!("Could not read Starship config file: {e}");
            return;
        }
    }

    let mut config_document = match Config::new_with_defaults(&starship_config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not parse Starship config file: {e:?}");
            return;
        }
    };

    config_document.set_theme(&theme_config);

    let starship_config_file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(false)
        .open(&starship_config_path);

    let write_result = match starship_config_file {
        Ok(mut f) => f.write_all(config_document.to_string().as_bytes()),
        Err(e) => {
            eprintln!("Could not open Starship config file {starship_config_path} to write: {e}");
            return;
        }
    };

    if let Err(e) = write_result {
        eprintln!("Could not write new config to Starship config file {starship_config_path}: {e}");
    }
}

fn get_random_theme_path(config_dir_path: &str) -> Option<String> {
    let file_names = get_file_names_of_directory(config_dir_path).ok()?;
    let theme_file_names = file_names
        .iter()
        .filter(|f| f.starts_with(PREFIX_THEMES_TOML))
        .filter(|f| f.ends_with(SUFFIX_THEMES_TOML))
        .collect::<Vec<&String>>();

    if theme_file_names.is_empty() {
        return None;
    }

    let random_index = random::get_fake(0, u32::try_from(theme_file_names.len()).ok()?)?;
    let random_file_name = theme_file_names[random_index as usize];

    Some(format!(
        "{config_dir_path}{DIRECTORY_SEPARATOR}{random_file_name}"
    ))
}
