/// Tries getting the **only** index of a `needle` in a `haystack`.
/// If `haystack` contains `needle` not exactly **one** time, `None` is returned.
///
/// # Arguments
///
/// * `haystack` - String to search for `needle`
/// * `needle` - The string that should be searched for in `haystack`
///
/// # Returns
///
/// Index of the **only** substring in `haystack` or `None`.
///
/// # Examples
///
/// ```
/// assert_eq!(get_single_index("Starships were meant to fly", "a"), None); // because "a" exists more than once
/// assert_eq!(get_single_index("Starships were meant to fly", "were"), Some(10));
/// ```
pub fn get_single_index(haystack: &str, needle: &str) -> Option<usize> {
    let matches = haystack
        .match_indices(needle)
        .take(2)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests_get_single_index {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(get_single_index("Starships were meant to fly", "a"), None);

        assert_eq!(
            get_single_index("Starships were meant to fly", "Hands up"),
            None
        );

        assert_eq!(
            get_single_index("Starships were meant to fly", "were"),
            Some(10)
        );
    }
}

#[macro_export]
/// Converts given ` &str` into `Option<String>`. If `&str` is empty or whitespace-only, it will result in `None`.
/// Otherwise, `Some(&str.trim())` will result.
///
/// # Arguments
///
/// * `&str` - The string to convert
///
/// # Examples
///
/// ```
/// assert!(optionize!("").is_none());
/// assert!(optionize!(" this is a test ")
///     .is_some_and(|trimmed| trimmed == "this is a test"));
/// ```
macro_rules! optionize {
    ( $str:expr ) => {{
        let trimmed = $str.trim();

        if trimmed != "" {
            Some(String::from(trimmed))
        } else {
            None
        }
    }};
}

#[cfg(test)]
mod tests_optionize {
    #[test]
    fn works() {
        assert!(optionize!("").is_none());

        assert!(optionize!(" this is a test ").is_some_and(|trimmed| trimmed == "this is a test"));
    }
}
