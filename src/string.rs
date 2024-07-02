#[macro_export]
/// Joins trimmed elements with given separator
///
/// # Arguments
///
/// * `separator`: (`&str`) - The string that will be used to join the given elements
/// * `...elements`: (`&str`) - The elements to join, if they're not empty
///
/// # Returns
///
/// Trimmed `String` of joined elements
///
/// # Example
///
/// ```
/// let result = join_trimmed!("-", "a", "", "b", "c", "\n");
/// assert_eq!(result, "a-b-c");
/// ```
macro_rules! join_trimmed {
    (  $sep:expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<&str> = Vec::new();
            $(
                let trimmed = $x.trim();
                if trimmed != "" {
                    temp_vec.push(trimmed);
                }
            )*
            temp_vec.join($sep)
        }
    };
}
