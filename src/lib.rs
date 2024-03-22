// Copyright © 2024 PDF OK (pdf_ok). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use colored::Colorize;
use std::fmt;
use std::path::PathBuf;

mod utils;
use crate::utils::read_file_data;

// PDFComposer struct
// #[derive(Debug)]
pub struct PDFComposer {
    fmy_source_files: Vec<PathBuf>,
    output_directory: PathBuf,
    // output_directory: Option<PathBuf>,
    pdf_version: Option<String>,
}

impl fmt::Debug for PDFComposer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFComposer")
            .field("fmy_source_files", &self.fmy_source_files)
            .field("output_directory", &self.output_directory)
            .field("pdf_version", &self.pdf_version)
            .finish()
    }
}

impl PDFComposer {
    /// Constructor function to create a new instance of PDFComposer
    pub fn new() -> Self {
        println!("{}\n", "PDF Composer new!".green().underline());
        // Create and return a new instance of PDFComposer. Setting default value, where applicable
        Self {
            fmy_source_files: Vec::new(), //<PathBuf>,
            output_directory: "pdf_composer_pdfs".into(),
            pdf_version: Some("1.7".to_string()),
        }
    }

    // TODO RL Remove later. Debug dev
    // Method to print all paths in the vector
    pub fn print_paths(&self) {
        println!("{}", "print_paths".cyan());
        for path in &self.fmy_source_files {
            println!("{:?}", path);
        }
        println!();
    }

    pub fn set_pdf_version(&mut self, pdf_version: &str) {
        self.pdf_version = Some(pdf_version.to_owned());
    }

    pub fn set_output_directory(&mut self, output_directory: &str) {
        self.output_directory = output_directory.into();
    }

    pub fn add_source_files(&mut self, paths: Vec<PathBuf>) {
        self.fmy_source_files.extend(paths);
    }

    pub fn generate_pdfs(&self) {
        println!("{}", "generate_pdfs".bright_green());

        // let optional_path = &self.output_directory;
        let path_string: String = <std::path::PathBuf as Clone>::clone(&self.output_directory).into_os_string().into_string().unwrap();

        // Now you can use path_string as a regular String variable
        println!("{}: {}", "Path as String".cyan(), path_string);

        // println!("{}: {:?}", "self.output_directory".bright_green(), self.output_directory);

        if self.fmy_source_files.is_empty() {
            panic!("{}", "No source files set".magenta().underline());
        }

        let bob = &self.fmy_source_files;
        // read_file_data(bob, self.output_directory);
        // read_file_data(bob);
    }
}

impl Default for PDFComposer {
    // Add a default implementation that creates an instance of PDFComposer with default values
    fn default() -> Self {
        // You can define default values for fields here
        // For demonstration purposes, let's assume all fields are set to default values
        Self::new()
    }
}

// Publicly expose the new function for creating instances of PDFComposer
pub mod pdf_composer {
    pub use super::PDFComposer;
}
