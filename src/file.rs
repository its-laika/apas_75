use std::{
    io::Error,
    path::{Path, PathBuf},
};

use crate::shell::Shell;

pub const DIRECTORY_SEPARATOR: char = '/';

const ENV_VAR_USER_HOME: &str = "HOME";
const SYMBOL_USER_HOME: char = '~';

/// Returns the file paths of a given directory `path`
///
/// # Arguments
///
/// * `path` - Directory path
///
/// # Returns
///
/// List of all non-directories (= files + symlinks) that are readable inside
/// given directory. Returns `Err` if directory cannot be opened.
pub fn get_files_of_directory(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let directory = path.read_dir()?;

    let file_names = directory
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().map(|t| !t.is_dir()).unwrap_or(false))
        .map(|f| f.path())
        .collect::<Vec<PathBuf>>();

    Ok(file_names)
}

/// Returns the canonicalized path by given `path`. Replaces "~" to the
/// current users' home directory, if possible.
///
/// Arguments
///
/// * `path` - The path that should be canonicalized
///
/// Returns
///
/// Full, canonicalized `PathBuf`, if possible. Returns `Err` if path cannot be
/// canonicalized.
pub fn build_canonicalized_path(path: &str) -> Result<PathBuf, Error> {
    let home_replaced_path = if path.starts_with(SYMBOL_USER_HOME) {
        let Some(home_dir) = Shell::get_env(ENV_VAR_USER_HOME) else {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("Environment variable '{ENV_VAR_USER_HOME}' not set (but required to canonicalize '{path}')"),
            ));
        };

        format!("{}{}{}", home_dir, DIRECTORY_SEPARATOR, &path[1..])
    } else {
        String::from(path)
    };

    Path::new(&home_replaced_path)
        .canonicalize()
        .map_err(|e| Error::new(e.kind(), format!("{e} for path '{path}'")))
}
