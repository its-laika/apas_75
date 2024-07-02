use std::{fs::File, io::Read};

use document::ConfigDocument;
use files::{build_full_path, get_file_names_of_directory, DIRECTORY_SEPARATOR};

mod document;
mod files;

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

    let theme_path = match get_random_theme_path(&config_dir_path) {
        Some(p) => p,
        None => return,
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

    let mut starship_config_file = match File::open(&starship_config_path) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Could not open Starship config file {starship_config_path}: {e}");
            return;
        }
    };

    let mut starship_config = String::new();
    if let Err(e) = starship_config_file.read_to_string(&mut starship_config) {
        eprint!("Could not read Starship config file: {e}");
        return;
    }

    let mut config_document = match ConfigDocument::new(&starship_config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not parse Starship config file: {:?}", e);
            return;
        }
    };

    config_document.set_theme(&theme_config);

    println!("{}", config_document);
}

fn get_random_theme_path(config_dir_path: &str) -> Option<String> {
    let file_names = match get_file_names_of_directory(config_dir_path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let theme_file_names = file_names
        .iter()
        .filter(|f| f.starts_with(PREFIX_THEMES_TOML))
        .filter(|f| f.ends_with(SUFFIX_THEMES_TOML));

    if let Some(file_name) = theme_file_names.into_iter().next() {
        // TODO: Pick random
        return Some(format!(
            "{}{}{}",
            config_dir_path, DIRECTORY_SEPARATOR, file_name
        ));
    }

    None
}
