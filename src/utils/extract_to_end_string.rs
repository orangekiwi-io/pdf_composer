// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// This function extracts a substring from a given string starting from the last occurrence of the specified delimiter.
///
/// # Arguments
///
/// * `input` - A string slice (`&str`) representing the original string.
/// * `delimiter_to_find` - A character (`char`) representing the delimiter to search for.
///
/// # Returns
///
/// * `Some(&str)` - A string slice representing the substring starting from the last occurrence of the delimiter. If the delimiter is not present, the entire original string is returned.
/// * `None` - If the last occurrence of the delimiter is the last character in the original string.
///
/// # Remarks
///
/// The function searches for the last occurrence of the specified delimiter using the `rfind` method.
///
/// If the delimiter is present, its last index is stored in `index`.
///
/// It then checks if this index is not the last character of the string. If it's not, `Some(&input[index + 1..])` is returned, which is a string slice starting from the character after the last delimiter until the end of the string.
///
/// If the delimiter is the last character of the string, `None` is returned.
///
/// If the delimiter is not present in the original string, the entire string is returned with `Some(input)`.
///
/// # Examples
///
/// ```
/// let input_string = "hello/world/foo/bar";
/// let delimiter = '/';
///
/// // Extracts "bar" from "hello/world/foo/bar"
/// assert_eq!(extract_to_end_string(input_string, delimiter), Some("bar"));
///
/// let input_string = "hello_world";
/// let delimiter = '/';
///
/// // Since there's no delimiter, the original string is returned
/// assert_eq!(extract_to_end_string(input_string, delimiter), Some("hello_world"));
///
/// let input_string = "hello/";
/// let delimiter = '/';
///
/// // Since the last delimiter is at the end, None is returned
/// assert_eq!(extract_to_end_string(input_string, delimiter), None);
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
