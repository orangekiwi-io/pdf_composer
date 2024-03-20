// Copyright © 2024 PDF OK (pdf_ok). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use std::path::PathBuf;

// PDFComposer struct
pub struct PDFComposer {
    // Add Front Matter (YAML) markdown files directly
    pub fmy_source_files: Vec<PathBuf>,
}

impl PDFComposer {
    /// Constructor function to create a new instance of PDFComposer
    pub fn new() -> Self {
        println!("PDF Composer new!");
        // Create and return a new instance of PDFComposer
        PDFComposer {
            fmy_source_files: Vec::new(),
        }
    }

    /// Method to add Front Matter (YAML) markdown files programmatically, one by one
    pub fn add_path(&mut self, path: PathBuf) {
        self.fmy_source_files.push(path);
    }

    // TODO RL Remove later. Debug dev
    // Method to print all paths in the vector
    pub fn print_paths(&self) {
        for path in &self.fmy_source_files {
            println!("{:?}", path);
        }
    }
}

impl Default for PDFComposer {
    // Add a default implementation that creates an instance of PDFComposer with default values
    fn default() -> Self {
        // You can define default values for fields here
        // For demonstration purposes, let's assume all fields are set to default values
        PDFComposer::new()
    }
}

// Publicly expose the new function for creating instances of PDFComposer
pub mod pdf_composer {
    pub use super::PDFComposer;
}
