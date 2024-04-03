// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use colored::Colorize;
use rayon::prelude::*;
use serde_yaml::Value;
use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

mod utils;
use utils::{merge_markdown_yaml, read_lines, yaml_mapping_to_btreemap};

mod core;
use core::build_pdf;

// PDFComposer struct
pub struct PDFComposer {
    fmy_source_files: Vec<PathBuf>,
    output_directory: PathBuf,
    pdf_version: PDFVersion,
    pdf_document_entries: Option<BTreeMap<String, String>>,
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

#[derive(Clone, Copy, Debug)]
pub enum PDFVersion {
    V1_7,
    V2_0,
}

impl ToString for PDFVersion {
    fn to_string(&self) -> String {
        match self {
            PDFVersion::V1_7 => String::from("1.7"),
            PDFVersion::V2_0 => String::from("2.0"),
        }
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

impl PDFComposer {
    /// Constructor function to create a new instance of PDFComposer
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

    pub fn set_pdf_version(&mut self, pdf_version: PDFVersion) {
        self.pdf_version = pdf_version;
    }

    pub fn set_output_directory(&mut self, output_directory: &str) {
        self.output_directory = output_directory.into();
    }

    pub fn add_source_files(&mut self, paths: Vec<PathBuf>) {
        self.fmy_source_files.extend(paths);
    }

    pub fn generate_pdfs(&self) {
        // TODO RL Find a nicer, more rusty, way to do early exits
        if self.fmy_source_files.is_empty() {
            panic!("{}", "No source files set".magenta().underline());
        }

        let number_of_files = &self.fmy_source_files.len();

        println!("{} {:#?}", "Files:".cyan(), &self.fmy_source_files);
        println!("Files to process: {}\n", number_of_files.to_string().cyan());

        self.fmy_source_files.par_iter().for_each(|document| {
            let mut rayon_yaml_delimiter_count = 0;
            let mut rayon_yaml_content: String = String::default();
            let mut rayon_markdown_content: String = String::default();
            let mut yaml_section_complete: bool = false;

            let filename = <std::path::PathBuf as Clone>::clone(document)
                .into_os_string()
                .into_string()
                .unwrap();

            match fs::metadata(filename.clone()) {
                Ok(_) => 'file_found: {
                    println!(
                        "File {} exists. {}",
                        filename.bright_cyan(),
                        "Reading...".bright_green()
                    );
                    if let Ok(lines) = read_lines(&filename) {
                        // Consumes the iterator, returns an (Optional) String
                        for line in lines.map_while(Result::ok) {
                            if line.trim() == "---" && rayon_yaml_delimiter_count < 2 {
                                rayon_yaml_delimiter_count += 1;
                            }

                            if line.trim() != "---" && rayon_yaml_delimiter_count < 2 {
                                rayon_yaml_content.push_str(&format!("{}{}", &line, "\n"));
                            }

                            if rayon_yaml_delimiter_count == 2 && !yaml_section_complete {
                                yaml_section_complete = true;
                                continue;
                            }

                            if rayon_yaml_delimiter_count == 2 && yaml_section_complete {
                                rayon_markdown_content.push_str(&format!("{}{}", &line, "\n"));
                            }
                        }
                    }

                    let yaml: Value = serde_yaml::from_str(&rayon_yaml_content).unwrap();
                    // println!("{}\n{:#?}", "rayon_yaml_delimiter_count".cyan(), rayon_yaml_delimiter_count);
                    // println!("{}\n{:#?}", "yaml Value".cyan(), yaml);

                    // if file exists, but is not a suitable yaml markdown file, early exit break
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

                    // // Convert Front Matter YAML to a BTreeMap
                    let yaml_btreemap: BTreeMap<String, Value> =
                        yaml_mapping_to_btreemap(&yaml).unwrap();
                    // println!("{}\n{:#?}", "yaml_btreemap".yellow(), yaml_btreemap);

                    // Insert Front Matter YAML into markdown (if applicable)
                    // TODO RL Add some sort of boolean check
                    let merged_markdown_yaml =
                        merge_markdown_yaml(yaml_btreemap.clone(), &rayon_markdown_content);

                    // Convert Markdown content to HTML
                    // markdown:: comes from the markdown crate
                    let html: String = markdown::to_html(&merged_markdown_yaml.to_owned());

                    // // Build PDF
                    let _ = build_pdf(
                        html,
                        filename.to_string(),
                        // extracted_filename.unwrap(),
                        yaml_btreemap,
                        self.output_directory.to_path_buf(),
                        <std::option::Option<
                            std::collections::BTreeMap<std::string::String, std::string::String>,
                        > as Clone>::clone(&self.pdf_document_entries)
                        .unwrap(),
                        self.pdf_version,
                    );

                    // Reset yaml and markdown content ready for the next file
                    // rayon_yaml_content = String::default();
                    // rayon_markdown_content = String::default();
                }
                Err(_) => {
                    println!("File {} not found.", filename.bright_red());
                }
            }
        });
    }

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
