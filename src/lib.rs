// Copyright © 2024 PDF OK (pdf_ok). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use std::{fmt, path::PathBuf};

use colored::Colorize;

// PDFComposer struct
pub struct PDFComposer {
    fmy_source_files: Vec<PathBuf>,
    // output_directory: Option<PathBuf>,
    output_directory: PathBuf,
    // pdf_version: Option<String>,
    pdf_version: String,
    pdf_document_entries: Option<Vec<PDFDocInfoEntry>>,
}

// NOTE: Don't forget to update debug_struct below when updating struct above
impl fmt::Debug for PDFComposer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFComposer")
            .field("fmy_source_files", &self.fmy_source_files)
            .field("output_directory", &self.output_directory)
            .field("pdf_version", &self.pdf_version)
            .field("pdf_document_entries", &self.pdf_document_entries)
            .finish()
    }
}

pub struct PDFDocInfoEntry {
    pub doc_info_entry: String,
    pub yaml_entry: String,
}

impl fmt::Debug for PDFDocInfoEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFDocInfoEntry")
            .field("doc_info_entry", &self.doc_info_entry)
            .field("yaml_entry", &self.yaml_entry)
            .finish()
    }
}

struct PDFDocEntries {
    dictionary: PDFDocInfoEntry,
}

impl PDFComposer {
    /// Constructor function to create a new instance of PDFComposer
    pub fn new() -> Self {
        println!("{}\n", "PDF Composer new!".green().underline());
        // Create and return a new instance of PDFComposer. Setting default value, where applicable
        Self {
            fmy_source_files: Vec::new(), //<PathBuf>,
            output_directory: "pdf_composer_pdfs".into(),
            pdf_version: "1.7".to_string(),
            // pdf_document_entries: Vec::new(),
            // pdf_document_entries: Some(Vec::new()),
            pdf_document_entries: None,
        }
    }

    // TODO RL Remove later. Debug dev
    // Method to print all paths in the vector
    pub fn print_paths(&self) {
        for path in &self.fmy_source_files {
            println!("{:?}", path);
        }
        println!();
    }

    pub fn set_pdf_version(&mut self, pdf_version: &str) {
        self.pdf_version = pdf_version.to_owned();
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
        let path_string: String = <std::path::PathBuf as Clone>::clone(&self.output_directory)
            .into_os_string()
            .into_string()
            .unwrap();

        // Now you can use path_string as a regular String variable
        println!("{}: {}", "Path as String".cyan(), path_string);

        // println!("{}: {:?}", "self.output_directory".bright_green(), self.output_directory);

        if self.fmy_source_files.is_empty() {
            panic!("{}", "No source files set".magenta().underline());
        }
    }

    pub fn set_doc_info_entry(&mut self, entry: PDFDocInfoEntry) {
    // pub fn set_doc_info_entry(&mut self, string_one: &str, string_two: &str) {
        let local_doc_info_entry = entry.doc_info_entry;
        let local_yaml_entry = entry.yaml_entry;

        match &mut self.pdf_document_entries {
            Some(vec) => {
                // Case where the Option contains Some variant
                vec.push(PDFDocInfoEntry {
                    doc_info_entry: local_doc_info_entry.clone(),
                    yaml_entry: local_yaml_entry.clone(),
                })

            }
            None => {
                // Case where the Option contains None variant
                self.pdf_document_entries = Some(vec![PDFDocInfoEntry {
                    doc_info_entry: local_doc_info_entry.clone(),
                    yaml_entry: local_yaml_entry.clone(),
                }]);
            }
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
