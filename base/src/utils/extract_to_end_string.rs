// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::path::MAIN_SEPARATOR;

/// /// Extracts a substring from a given string starting from the last occurrence of the OS-specific delimiter (MAIN_SEPARATOR).
///
/// # Arguments
///
/// * `input` - A string from which the substring will be extracted.
///
/// # Returns
///
/// An option containing the extracted substring, or None if the string does not contain the delimiter.
///
/// # Examples
///
/// ```
/// // Import the function into scope
/// use your_module::extract_to_end_string;
///
/// // Define an input string containing directory path
/// let input_string = "/path/to/some/directory/";
///
/// // Extracts "directory/" from "/path/to/some/directory/"
/// assert_eq!(extract_to_end_string(input_string), Some("directory/"));
///
/// let input_string = "/path/to/some/file.txt";
///
/// // Since there's no delimiter, the original string is returned
/// assert_eq!(extract_to_end_string(input_string), Some("/path/to/some/file.txt"));
///
/// let input_string = "/";
///
/// // Since the last delimiter is the last character in the string, None is returned
/// assert_eq!(extract_to_end_string(input_string), None);
/// ```
pub fn extract_to_end_string(input: &str) -> Option<&str> {
    if let Some(index) = input.rfind(MAIN_SEPARATOR) {
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
