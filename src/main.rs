use std::{io::Error, path::PathBuf, process::ExitCode};

use file::{build_canonicalized_path, get_files_of_directory};
use shell::{CommandBuilder, Shell, ENV_KEY_STARSHIP_CONFIG};

mod file;
mod shell;

const CONFIG_DIR: &str = "~/.config/apas_75/";
const FILE_EXTENSION_TOML: &str = "toml";

fn main() -> ExitCode {
    let Some(shell) = Shell::get_current() else {
        eprintln!("Could not determine current shell");
        return ExitCode::FAILURE;
    };

    // TODO: Config dir as an argument?
    let configs_path = match build_canonicalized_path(CONFIG_DIR) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not canonicalize path '{CONFIG_DIR}': {e}");
            return ExitCode::FAILURE;
        }
    };

    let files = match get_files_of_directory(configs_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Could not get files of path '{}': {e}",
                configs_path.to_str().unwrap_or("[FAILED TO UNWRAP PATH]")
            );
            return ExitCode::FAILURE;
        }
    };

    let current_file_name = Shell::get_env(ENV_KEY_STARSHIP_CONFIG);

    let theme_path = match get_next_theme_file_name(&files, current_file_name) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "Could not determine next configuration in {}: {e}",
                configs_path.to_str().unwrap_or("[FAILED TO UNWRAP PATH]")
            );
            return ExitCode::FAILURE;
        }
    };

    match shell.build_env_command(
        ENV_KEY_STARSHIP_CONFIG,
        theme_path.to_str().expect("Could not unwrap theme path"),
    ) {
        Ok(c) => print!("{c}"),
        Err(e) => {
            eprintln!("Could not build env command: {e}");
        }
    }

    ExitCode::SUCCESS
}

fn get_next_theme_file_name(
    files: &[PathBuf],
    env_config_path: Option<String>,
) -> Result<PathBuf, Error> {
    let mut theme_files = files
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

    if files.len() == 1 {
        return Ok(first);
    }

    let Some(current_path) = env_config_path else {
        return Ok(first);
    };

    let next = theme_files.iter().find(|&f| match f.to_str() {
        Some(s) => s > &current_path,
        None => false,
    });

    match next {
        Some(&s) => Ok(s.clone()),
        None => Ok(first),
    }
}
