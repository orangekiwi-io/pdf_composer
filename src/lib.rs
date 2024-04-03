// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use colored::Colorize;
use rayon::prelude::*;
use serde_yaml::Value;
use std::{collections::BTreeMap, fmt, fs, path::PathBuf, process};

mod utils;
use utils::{merge_markdown_yaml, read_lines, yaml_mapping_to_btreemap};

mod core;
use core::build_pdf;

pub const CHECK_MARK: &str = "\u{2713} ";
pub const CROSS_MARK: &str = "\u{2717} ";

/// PDFComposer struct represents a tool for composing PDF documents from multiple source files.
pub struct PDFComposer {
    /// Vector containing paths to the source files used for composing the PDF document.
    fmy_source_files: Vec<PathBuf>,
    /// Path to the directory where the composed PDF document will be saved.
    output_directory: PathBuf,
    /// Specifies the version of the PDF format to be used.
    pdf_version: PDFVersion,
    /// Optional mapping of document entries, where the key represents the entry name and the value represents the content.
    pdf_document_entries: Option<BTreeMap<String, String>>,
}

impl fmt::Debug for PDFComposer {
    /// Implements the Debug trait for the PDFComposer struct, allowing it to be formatted for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFComposer")
            .field("fmy_source_files", &self.fmy_source_files)
            .field("output_directory", &self.output_directory)
            .field("pdf_version", &self.pdf_version)
            .field("pdf_document_entries", &self.pdf_document_entries)
            .finish()
    }
}

/// Enum to represent different versions of the PDF specification.
/// See here: https://pdfa.org/resource/pdf-specification-archive/
#[derive(Clone, Copy, Debug)]
pub enum PDFVersion {
    /// Represents PDF version 1.7.
    V1_7,
    /// Represents PDF version 2.0.
    V2_0,
}

impl ToString for PDFVersion {
    /// Implements the ToString trait for PDFVersion, converting enum variants to their corresponding String representations.
    fn to_string(&self) -> String {
        match self {
            PDFVersion::V1_7 => String::from("1.7"),
            PDFVersion::V2_0 => String::from("2.0"),
        }
    }
}

/// Struct representing an entry in the PDF document information.
/// Used to set everything up ready for lopdf
pub struct PDFDocInfoEntry {
    /// The name of the document information entry.
    pub doc_info_entry: String,
    /// The corresponding YAML entry associated with the document information.
    pub yaml_entry: String,
}

impl fmt::Debug for PDFDocInfoEntry {
    /// Implements the Debug trait for the PDFDocInfoEntry struct, allowing it to be formatted for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFDocInfoEntry")
            .field("doc_info_entry", &self.doc_info_entry)
            .field("yaml_entry", &self.yaml_entry)
            .finish()
    }
}

impl PDFComposer {
    /// Constructor function to create a new instance of PDFComposer with default values.
    pub fn new() -> Self {
        // Create and return a new instance of PDFComposer.
        // Setting default values, where applicable.
        Self {
            fmy_source_files: Vec::new(),
            output_directory: "pdf_composer_pdfs".into(),
            pdf_version: PDFVersion::V1_7,
            pdf_document_entries: None,
        }
    }

    /// Sets the PDF version for the PDFComposer instance.
    pub fn set_pdf_version(&mut self, pdf_version: PDFVersion) {
        self.pdf_version = pdf_version;
    }

    /// Sets the output directory for the generated PDF documents.
    pub fn set_output_directory(&mut self, output_directory: &str) {
        self.output_directory = output_directory.into();
    }

    /// Adds source files to the PDFComposer instance for processing.
    pub fn add_source_files(&mut self, paths: Vec<PathBuf>) {
        self.fmy_source_files.extend(paths);
    }

    /// Generates PDF documents based on the configured settings and source files.
    pub fn generate_pdfs(&self) {
        // Handle case where no source files are set.
        let error_message = "".to_owned()
            + &CROSS_MARK.on_red().to_string()
            + &"No source files set.".on_red().to_string()
            + " Exiting\n";
        if self.fmy_source_files.is_empty() {
            eprintln!("{}", error_message);
            process::exit(0);
        }

        println!("{} {:#?}", "Files:".cyan(), &self.fmy_source_files);
        println!(
            "Files to process: {}\n",
            &self.fmy_source_files.len().to_string().cyan()
        );

        // Process each source file in parallel.
        self.fmy_source_files.par_iter().for_each(|document| {
            // Initialize variables for processing YAML and Markdown content.
            let mut rayon_yaml_delimiter_count = 0;
            let mut rayon_yaml_content: String = String::default();
            let mut rayon_markdown_content: String = String::default();
            let mut yaml_section_complete: bool = false;

            // Extract filename from PathBuf.
            let filename = <std::path::PathBuf as Clone>::clone(document)
                .into_os_string()
                .into_string()
                .unwrap();

            // Attempt to read metadata of the file.
            match fs::metadata(filename.clone()) {
                Ok(_) => 'file_found: {
                    // File exists, proceed with reading.
                    println!(
                        "File {} exists. {}",
                        filename.bright_cyan(),
                        "Reading...".bright_green()
                    );
                    if let Ok(lines) = read_lines(&filename) {
                        // Iterate through lines and process YAML and Markdown content.
                        for line in lines.map_while(Result::ok) {
                            // Check YAML delimiters and extract content.
                            if line.trim() == "---" && rayon_yaml_delimiter_count < 2 {
                                rayon_yaml_delimiter_count += 1;
                            }

                            if line.trim() != "---" && rayon_yaml_delimiter_count < 2 {
                                rayon_yaml_content.push_str(&format!("{}{}", &line, "\n"));
                            }

                            // Check if YAML section is complete.
                            if rayon_yaml_delimiter_count == 2 && !yaml_section_complete {
                                yaml_section_complete = true;
                                continue;
                            }

                            // Extract Markdown content after YAML section.
                            if rayon_yaml_delimiter_count == 2 && yaml_section_complete {
                                rayon_markdown_content.push_str(&format!("{}{}", &line, "\n"));
                            }
                        }
                    }

                    // Parse YAML content.
                    let yaml: Value = serde_yaml::from_str(&rayon_yaml_content).unwrap();
                    // Check if YAML is valid.
                    // If file exists, but is not a suitable yaml markdown file, early exit break
                    if rayon_yaml_delimiter_count == 0 || yaml == Value::Null {
                        println!("File {} is not a valid yaml file", filename.bright_red());
                        break 'file_found;
                    } else {
                        println!(
                            "{}. {}",
                            filename.bright_cyan(),
                            "Processing...".bright_green()
                        );
                    }

                    // Convert YAML Front Matter to a BTreeMap.
                    let yaml_btreemap: BTreeMap<String, Value> =
                        yaml_mapping_to_btreemap(&yaml).unwrap();

                    // Insert YAML Front Matter into markdown.
                    let merged_markdown_yaml =
                        merge_markdown_yaml(yaml_btreemap.clone(), &rayon_markdown_content);

                    // Convert Markdown content to HTML.
                    // markdown:: comes from the markdown crate
                    let html: String = markdown::to_html(&merged_markdown_yaml.to_owned());

                    // Build the PDF document.
                    let _ = build_pdf(
                        html,
                        filename.to_string(),
                        yaml_btreemap,
                        self.output_directory.to_path_buf(),
                        <std::option::Option<
                            std::collections::BTreeMap<std::string::String, std::string::String>,
                        > as Clone>::clone(&self.pdf_document_entries)
                        .unwrap(),
                        self.pdf_version,
                    );
                }
                Err(_) => {
                    // File not found, print error message.
                    println!("File {} not found.", filename.bright_red());
                }
            }
        });
    }

    /// Sets a document information entry for the PDFComposer instance.
    pub fn set_doc_info_entry(&mut self, entry: PDFDocInfoEntry) {
        // Reserved metadata entries in the document information dictionary
        // These are case sensitive and must be capitalised.
        // All others will be as entered by the user.
        let local_doc_info_entry: String = match entry.doc_info_entry.to_lowercase().as_str() {
            "title" => "Title".to_string(),
            "author" => "Author".to_string(),
            "subject" => "Subject".to_string(),
            "keywords" => "Keywords".to_string(),
            _ => entry.doc_info_entry.to_string(),
        };
        let local_yaml_entry = entry.yaml_entry;

        // Match and handle the Option variant to insert the entry into the PDF document entries.
        match &mut self.pdf_document_entries {
            Some(map) => {
                // Case where the Option contains Some variant
                map.insert(local_doc_info_entry.clone(), local_yaml_entry.clone());
            }
            None => {
                // Case where the Option contains None variant
                let mut new_map = BTreeMap::new();
                new_map.insert(local_doc_info_entry.clone(), local_yaml_entry.clone());
                self.pdf_document_entries = Some(new_map);
            }
        }
    }
}

impl Default for PDFComposer {
    /// Implements the Default trait for PDFComposer, providing a default implementation to create an instance of PDFComposer with default values.
    fn default() -> Self {
        // Creates and returns a new instance of PDFComposer with default values.
        Self::new()
    }
}

/// Module for publicly exposing the PDFComposer struct and its associated functions.
pub mod pdf_composer {
    /// Re-exports the PDFComposer struct for public use.
    pub use super::PDFComposer;
}
