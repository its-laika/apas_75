use std::{
    fs::OpenOptions,
    io::{Error, Write},
};

use files::{build_full_path, get_files_of_directory};
use toml::{merge_toml_documents, open_toml_document};

mod files;
mod toml;

const CONFIG_PATH: &str = "starship.toml";
const PREFIX_THEMES_TOML: &str = "theme-";
const SUFFIX_THEMES_TOML: &str = ".toml";
const DIRECTORY_SEPARATOR: char = '/';

fn main() -> Result<(), Error> {
    let theme_toml_paths = get_files_of_directory(".")?
        .into_iter()
        .filter(|p| {
            let file_name = p.split(DIRECTORY_SEPARATOR).last();
            match file_name {
                Some(f) => f.starts_with(PREFIX_THEMES_TOML) && f.ends_with(SUFFIX_THEMES_TOML),
                None => false,
            }
        })
        .collect::<Vec<String>>();

    let theme_toml_path = match theme_toml_paths.first() {
        Some(p) => p,
        None => return Ok(()),
    };

    let target_config_path = build_full_path(CONFIG_PATH)?;
    let mut target_config = open_toml_document(&target_config_path)?;
    let theme_config = open_toml_document(theme_toml_path)?;

    merge_toml_documents(&mut target_config, &theme_config);

    let themed_config_content = target_config.to_string();

    let mut target_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(false)
        .open(target_config_path)?;

    target_file.write_all(themed_config_content.as_bytes())?;
    target_file.flush()?;

    Ok(())
}
