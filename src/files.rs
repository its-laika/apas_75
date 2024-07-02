use std::{env, fs::read_dir, io::Error, path::absolute};

pub const DIRECTORY_SEPARATOR: char = '/';
const ENV_VAR_USER_HOME: &str = "HOME";
const SYMBOL_USER_HOME: char = '~';

pub fn get_file_names_of_directory(path: &str) -> Result<Vec<String>, Error> {
    let directory = read_dir(path)?;

    let mut file_names = Vec::new();
    for dir_entry_result in directory {
        let dir_entry = match dir_entry_result {
            Ok(d) => d,
            Err(_) => continue,
        };

        let file_type = match dir_entry.file_type() {
            Ok(f) => f,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            continue;
        }

        let file_name = dir_entry.file_name();
        if let Some(p) = file_name.to_str() {
            file_names.push(String::from(p));
        }
    }

    Ok(file_names)
}

pub fn build_full_path(path: &str) -> Result<String, Error> {
    let home_replaced_path = if !path.starts_with(SYMBOL_USER_HOME) {
        String::from(path)
    } else {
        let home_dir = env::var(ENV_VAR_USER_HOME).map_err(|_| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "HOME environment variable not set",
            )
        })?;

        format!("{}{}{}", home_dir, DIRECTORY_SEPARATOR, &path[1..])
    };

    return absolute(home_replaced_path).and_then(|path_buf| {
        path_buf.to_str().map(String::from).ok_or(Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not build absolute path",
        ))
    });
}
