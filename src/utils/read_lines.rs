// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::{BufRead, BufReader, Lines, Result};
use std::{fs::File, path::Path};

/// This function reads the lines of a file and returns an iterator over the lines.
///
/// # Arguments
///
/// * `filename` - A value that implements the `AsRef<Path>` trait, which represents the path
///   to the file to be read. This could be a `String`, `&str`, or `PathBuf`.
///
/// # Returns
///
/// * `Ok(Lines<BufReader<File>>)` - An iterator over the lines of the file, wrapped in a `BufReader`
///   for efficient reading.
/// * `Err(e)` - An error if the file cannot be opened, where `e` is an instance of `std::io::Error`.
///
/// # Remarks
///
/// This function uses the `std::fs::File` module to open the file specified by `filename`.
/// If the file is successfully opened, it creates a `BufReader` instance around the file,
/// which provides buffered reading capabilities for improved performance.
///
/// The `lines()` method is then called on the `BufReader`, which returns an iterator over
/// the lines of the file. Each line is returned as a `Result<String>`, where an `Err` value
/// indicates an error reading the line (e.g., an invalid UTF-8 encoding).
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// // Define the path to the file
/// let file_path = Path::new("example.txt");
///
/// // Read lines from the file
/// match read_lines(file_path) {
///     Ok(lines) => {
///         // Iterate over the lines and print them
///         for line in lines {
///             if let Ok(line) = line {
///                 println!("{}", line);
///             }
///         }
///     }
///     Err(err) => {
///         eprintln!("Error reading file: {}", err);
///     }
/// }
/// ```
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    // Open the file specified by `filename`
    let file = File::open(filename)?;

    // Create a `BufReader` instance around the opened file
    // and return an iterator over the lines of the file
    Ok(BufReader::new(file).lines())
}