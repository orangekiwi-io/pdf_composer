// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/orangekiwi-io/pdf_composer/main/assets/PDFComposer.png"
)]
//! > **PDF generation from Yaml Front Matter documents for Rust**
//!
//! ## Overview
//! This crate creates a PDF document from YAML Front Matter source documents. The YAML values can be used for PDF Dictionary entries or used to replace placeholder references within the Markdown section of the YAML Front Matter document.
//!
//! ## Features
//!
//! ### PDF output destination
//! Generated PDFs are saved to an output destination directory relative to your project root.
//!
//! If no output destination is set, then PDFs will be saved in a directory called `pdf_composer_pdfs`. This stops the root of your project being littered with generated PDFs. Nice and tidy.
//!
//! ### PDF versions
//! Currently only the latest two versions of the PDF specifications are supported (and encouraged), namely versions 1.7 and 2.0.
//!
//! If not PDF version is set, then version 1.7 is used by default.
//!
//! **NOTE:** This is not to be confused with the version/edition of the PDF itself. That is something for you to decide (and maybe set as a YAML value for inclusion as a PDF Dictionary entry, see below for more information).
//!
//! To set the version see the table showing the corresponding enum with version number.
//!
//! | enum | PDF Version |
//! | --- | --- |
//! | PDFVersion::V1_7 | 1.7 |
//! | PDFVersion::V2_0 | 2.0 |
//!
//! ## PDF Dictionary entries
//!
//! PDF Dictionary entries are those Name and Value pairs you can see if you selcted "Document Properties" within a PDF Reader on a PDF document. Dictionary entries are case sensitive, with a few reserved names.
//!
//! ### Reserved Dictionary Entries
//! * Title
//! * Author
//! * Subject
//! * Keywords
//!
//! These **must** be capitalised. **PDFComposer** automatically captialises the reserved named ones only. All others will be left as entered.
//!
//! In **PDFComposer** The Title entry is a special case. As part of the PDF generation process, the `title` value from the YAML document is automatically inserted into the `<title>` tag in the HTML templates used. As a result, the Dictionary entry is populated. If no YAML value is found, then the filename of the source file will be used instead.
//!
//! Empty entries are **not** allowed. If no corresponding YAML entry can be found, then an empty entry will not be added to the PDF document.
//!
//! For example, if you want to set a Dictionary entry called `Language` and you set it to a YAML entry that does not exist in the YAML document, **PDFComposer** will not create an empty entry.
//!
//! ### Example for setting a Dictionary entry
//! ```
//! PDFDocInfoEntry {
//!     doc_info_entry: "Subject".to_owned(),
//!     yaml_entry: "description".to_owned(),
//! }
//! ```
//!
//! `doc_info_entry` is the PDF Dictionary entry.
//!
//! `yaml_entry` is the YAML value that will be assigned to the Dictionary entry.
//!
//! ## YAML Markdown placeholder substitution
//!
//! It is possible to simple substitution within the markdown section of the YAML document. This is possible by using `{{my_yaml_value}}` within the markdown section.
//!
//! ### Example
//! ```
//! ---
//!
//! # Front Matter (YAML)
//!
//! author: "Richard"
//!
//! ---
//!
//! The author of this document is {{author}}.
//!
//! ```
//!
//! The result will be: `The author of this document is Richard.`
//!
//! If the YAML value cannot be found, then the substitution placeholder will remain in the generated output.
//!
//! ### Example
//! ```
//! ---
//!
//! # Front Matter (YAML)
//!
//! author: "Richard"
//!
//! ---
//!
//! The author of this document is {{name}}.
//!
//! ```
//!
//! The result will be: `The author of this document is {{name}}.`
//!
//! ## Example usage
//!
//! Assuming you have Rust up and running (tested with rust verion `1.76+`) and you have run `cargo add pdf_composer` to install the **PDFComposer** crate, then you can begin.
//!
//! ```
//! use pdf_composer::{PDFComposer, PDFDocInfoEntry, PDFVersion};
//!
//! // Create a new PDFComposer instance
//! let mut my_pdf_instance = PDFComposer::new();
//!
//! // Add some paths. Relative paths
//! let paths = vec![
//!     PathBuf::from("source_mds/file_01.md"),
//!     PathBuf::from("source_mds/file_02.md")
//! ];
//! my_pdf_instance.add_source_files(paths);
//!
//! // PDF version (not the version of the document, but the Adobe (formerly) PDF format version)
//! my_pdf_instance.set_pdf_version(PDFVersion::V2_0);
//!
//! // Output directory for the generated PDFs
//! my_pdf_instance.set_output_directory("example_pdfs");
//!
//! // Metadata for the PDFs
//! let author_entry = PDFDocInfoEntry {
//!     doc_info_entry: "Author",
//!     yaml_entry: "author",
//! };
//! let keywords_entry = PDFDocInfoEntry {
//!     doc_info_entry: "Keywords",
//!     yaml_entry: "keywords",
//! };
//! let subject_entry = PDFDocInfoEntry {
//!     doc_info_entry: "Subject",
//!     yaml_entry: "description",
//! };
//! let language_entry = PDFDocInfoEntry {
//!     doc_info_entry: "Language",
//!     yaml_entry: "language",
//! };
//! my_pdf_instance.set_doc_info_entry(author_entry);
//! my_pdf_instance.set_doc_info_entry(keywords_entry);
//! my_pdf_instance.set_doc_info_entry(subject_entry);
//! my_pdf_instance.set_doc_info_entry(language_entry);
//!
//! // Generate the PDF(s)
//! my_pdf_instance.generate_pdfs();
//!
//! ```
//!
//! ## License
//!
//! The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/)
//! - [MIT license](https://opensource.org/licenses/MIT)
//!
//! ## Future plans
//!
//! Some ideas, but not limited to:
//! * Set paper sizes, orientation and margins
//! * HTML templates
//! * Pretty default print stylesheets
//! * Allow for direct String values to be used for PDF Dictionary entries without having to have a YAML value first
//! * Combine multiple YAML Front Matter documents into one generated PDF document
//! * Pagination and page numbers

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

/// CONST for a tick/check mark character plus a space character
pub const CHECK_MARK: &str = "\u{2713} ";
/// CONST for a cross character plus a space character
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
/// See <https://pdfa.org/resource/pdf-specification-archive/> for more information on the PDF specifications
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
/// Used to set things up ready for lopdf
pub struct PDFDocInfoEntry<'a> {
    /// The name of the document information entry.
    pub doc_info_entry: &'a str,
    /// The corresponding YAML entry associated with the document information.
    pub yaml_entry: &'a str,
}

impl<'a> fmt::Debug for PDFDocInfoEntry<'a> {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDFComposer instance with default values
    /// let my_pdf_doc = PDFComposer::new();
    /// ```
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
    /// Sets the PDF version for the PDF document.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::{PDFComposer, PDFVersion};
    ///
    /// // Create a new PDF document
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the PDF version to 2.0
    /// my_pdf_doc.set_pdf_version(PDFVersion::V2_0);
    /// ```
    pub fn set_pdf_version(&mut self, pdf_version: PDFVersion) {
        self.pdf_version = pdf_version;
    }

    /// Sets the output directory for the generated PDF documents.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the output directory to "output/pdf"
    /// my_pdf_doc.set_output_directory("output/pdf");
    /// ```
    pub fn set_output_directory(&mut self, output_directory: &str) {
        self.output_directory = output_directory.into();
    }

    /// Adds source files to the PDFComposer instance for processing.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    /// use std::path::PathBuf;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Define paths to source files
    /// let source_files = vec![
    ///     PathBuf::from("source/file1.txt"),
    ///     PathBuf::from("source/file2.txt"),
    /// ];
    ///
    /// // Add the source files to the PDF generator
    /// my_pdf_doc.add_source_files(source_files);
    /// ```
    pub fn add_source_files(&mut self, paths: Vec<PathBuf>) {
        self.fmy_source_files.extend(paths);
    }

    /// Sets a document information entry for the PDFComposer instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::{PDFComposer, PDFDocInfoEntry};
    ///
    /// // Create a new PDFComposer instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Define a document information entry
    /// let doc_info_entry = PDFDocInfoEntry {
    ///     doc_info_entry: "Author",
    ///     yaml_entry: "author",
    /// };
    ///
    /// // Set the document information entry in the PDFComposer
    /// my_pdf_doc.set_doc_info_entry(doc_info_entry);
    /// ```
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
                map.insert(local_doc_info_entry.clone(), local_yaml_entry.to_owned());
            }
            None => {
                // Case where the Option contains None variant
                let mut new_map = BTreeMap::new();
                new_map.insert(local_doc_info_entry.clone(), local_yaml_entry.to_owned());
                self.pdf_document_entries = Some(new_map);
            }
        }
    }

    /// Generates PDF documents based on the configured settings and source files.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a PDF generator instance
    /// let my_pdf_doc = PDFComposer::new();
    ///
    /// // Generate PDFs based on the configuration and source files
    /// my_pdf_doc.generate_pdfs();
    /// ```
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
}

impl Default for PDFComposer {
    /// Implements the Default trait for PDFComposer, providing a default implementation to create an instance of PDFComposer with default values.
    fn default() -> Self {
        // Creates and returns a new instance of PDFComposer with default values.
        Self::new()
    }
}
