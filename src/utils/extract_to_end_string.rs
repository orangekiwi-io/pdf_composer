/// Extracts a substring from the last occurrence of a specified delimiter to the end of the input string.
///
/// # Arguments
///
/// * `input` - The input string from which to extract the substring.
/// * `delimiter_to_find` - The character delimiter to search for.
///
/// # Returns
///
/// Returns `Some(&str)` containing the substring from the last occurrence of the delimiter to the end of the input string,
/// or `None` if the delimiter is not found or if it is the last character in the input string.
///
/// # Example
///
/// ```
/// use your_crate_name::extract_to_end_string;
///
/// let input = "my_string_to_test/path/to/filename";
/// let delimiter_to_find = '/';
/// let result = extract_to_end_string(input, delimiter_to_find);
/// assert_eq!(result, Some("filename"));
/// ```
pub fn extract_to_end_string(input: &str, delimiter_to_find: char) -> Option<&str> {
    if let Some(index) = input.rfind(delimiter_to_find) {
        // Check if the index is not the last character in the string
        if index < input.len() - 1 {
            // Use the index to get the substring after the last delimiter
            Some(&input[index + 1..])
        } else {
            // If the last delimiter is the last character in the string, return None
            None
        }
    } else {
        // If there is no delimiter, return the original string
        Some(input)
    }
}