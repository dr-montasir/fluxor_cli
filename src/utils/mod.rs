/// Creates a valid Rust crate name from an input string.
///
/// This function normalizes a string by replacing non-alphanumeric characters with underscores,
/// ensuring the string doesn't start with a number, and handling empty input.
///
/// # Arguments
///
/// * `input` - The string to normalize.
///
/// # Returns
///
/// A valid Rust crate name (a string that doesn't start with a number and only contains alphanumeric characters or underscores).
/// Returns an empty string if the input is empty or cannot be converted to a valid crate name.
///
/// # Examples
///
/// ```rust
/// use fluxor_cli::utils::to_crate_name;
///
/// assert_eq!(to_crate_name("my-crate"), "my_crate");
/// assert_eq!(to_crate_name("123crate"), "_123crate");
/// assert_eq!(to_crate_name(""), "");
/// assert_eq!(to_crate_name("crate-with-special-characters!"), "crate_with_special_characters");
/// assert_eq!(to_crate_name("validName"), "validname");
/// ```
pub fn to_crate_name(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(input.len());
    let mut last_char_was_underscore = false;

    for c in input.chars() {
        if c.is_alphanumeric() {
            output.push(c);
            last_char_was_underscore = false;
        } else if c.is_ascii_whitespace() {
            // skip whitespace
        } else {
            // Replace other characters with underscore if last char wasn't underscore
            if !last_char_was_underscore && !output.is_empty() {
                output.push('_');
                last_char_was_underscore = true;
            }
        }
    }

    // Remove trailing underscores for cleaner names (optional)
    while output.ends_with('_') {
        output.pop();
    }

    if output.is_empty() {
        return String::new();
    }

    // Prepend underscore if first character is a digit
    if output.chars().next().unwrap().is_ascii_digit() {
        output.insert(0, '_');
    }

    // Convert to lowercase for consistency
    output = output.to_lowercase();

    output
}