use std::{env, io::Error, path::PathBuf, process::ExitCode};

use file::{build_canonicalized_path, get_files_of_directory};
use shell::{CommandBuilder, Shell, ENV_KEY_STARSHIP_CONFIG};
use starship::get_next_theme_file_name;

mod file;
mod shell;
mod starship;

const DEFAULT_CONFIG_DIR: &str = "~/.config/apas_75/";
const FILE_EXTENSION_TOML: &str = "toml";

fn main() -> ExitCode {
    let Some(shell) = Shell::get_current() else {
        eprintln!("Could not determine current shell");
        return ExitCode::FAILURE;
    };

    let configs_path = match build_config_directory() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not canonicalize config path: {e}");
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

/// Determines and builds `apas_75` config directory.
///
/// Checks for given command line argument or falls back to default path
/// and returns canonicalized path or `Err` on error.
///
/// # Examples
///
/// ```rust
/// let Ok(dir) = build_config_directory() or else {
///     println!("Could not build config directory");
///     return;
/// }
///
/// println!("Using config directory: {:?}", dir);
/// ```
fn build_config_directory() -> Result<PathBuf, Error> {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => DEFAULT_CONFIG_DIR.to_string(),
    };

    build_canonicalized_path(&path)
}
