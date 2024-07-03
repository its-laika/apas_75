use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::{
    document::Config, files::get_file_names_of_directory, files::DIRECTORY_SEPARATOR, random,
};

use super::error::ReplacementError;

const FILE_EXTENSION_TOML: &str = ".toml";

pub fn replace_custom_theme(
    starship_config_path: &str,
    config_directory_path: &str,
    theme_file_prefix: &str,
) -> Result<(), ReplacementError> {
    let Some(theme_path) = get_random_theme_path(config_directory_path, theme_file_prefix) else {
        return Ok(());
    };

    let Ok(mut theme_config_file) = File::open(&theme_path) else {
        return Err(ReplacementError::NotReadable(theme_path));
    };

    let mut theme_config = String::new();
    if theme_config_file.read_to_string(&mut theme_config).is_err() {
        return Err(ReplacementError::NotReadable(theme_path));
    }

    let mut starship_config = String::new();
    {
        let Ok(mut starship_config_file) = File::open(starship_config_path) else {
            return Err(ReplacementError::NotReadable(String::from(
                starship_config_path,
            )));
        };

        if starship_config_file
            .read_to_string(&mut starship_config)
            .is_err()
        {
            return Err(ReplacementError::NotReadable(String::from(
                starship_config_path,
            )));
        }
    }

    let Ok(mut config_document) = Config::new_with_defaults(&starship_config) else {
        return Err(ReplacementError::InvalidConfig);
    };

    config_document.set_theme(&theme_config);

    let starship_config_file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(false)
        .open(starship_config_path);

    let write_result = match starship_config_file {
        Ok(mut f) => f.write_all(config_document.to_string().as_bytes()),
        Err(_) => {
            return Err(ReplacementError::NotWriteable(String::from(
                starship_config_path,
            )))
        }
    };

    if write_result.is_err() {
        return Err(ReplacementError::NotReadable(String::from(
            starship_config_path,
        )));
    }

    Ok(())
}

fn get_random_theme_path(config_dir_path: &str, theme_file_prefix: &str) -> Option<String> {
    let file_names = get_file_names_of_directory(config_dir_path).ok()?;
    let theme_file_names = file_names
        .iter()
        .filter(|f| f.starts_with(theme_file_prefix))
        .filter(|f| f.ends_with(FILE_EXTENSION_TOML))
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
