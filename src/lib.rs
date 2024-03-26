// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use colored::Colorize;
use serde_yaml::Value;
use std::{collections::BTreeMap, fmt, path::PathBuf};

mod utils;
use utils::{merge_markdown_yaml, read_lines, yaml_mapping_to_btreemap};

mod core;
use core::build_pdf;

use crate::utils::extract_to_end_string;

// PDFComposer struct
pub struct PDFComposer {
    fmy_source_files: Vec<PathBuf>,
    output_directory: PathBuf,
    pdf_version: String,
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
        println!("{}\n", "PDF Composer new!".green().underline());
        // Create and return a new instance of PDFComposer. Setting default value, where applicable
        Self {
            fmy_source_files: Vec::new(),
            output_directory: "pdf_composer_pdfs".into(),
            pdf_version: "1.7".to_string(),
            pdf_document_entries: None,
        }
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

        let path_string: String = <std::path::PathBuf as Clone>::clone(&self.output_directory)
            .into_os_string()
            .into_string()
            .unwrap();

        // Now you can use path_string as a regular String variable
        println!("{}: {}", "Output path as String".cyan(), path_string);

        if self.fmy_source_files.is_empty() {
            panic!("{}", "No source files set".magenta().underline());
        }

        let mut file = 0;
        let mut yaml_delimiter_count = 0;
        let mut yaml_content: String = String::default();
        let mut markdown_content: String = String::default();
        println!("Files to process: {}", &self.fmy_source_files.len());
        println!("{:#?}", &self.fmy_source_files);

        while file < self.fmy_source_files.len() {
            let filename = &<std::path::PathBuf as Clone>::clone(&self.fmy_source_files[file])
                .into_os_string()
                .into_string()
                .unwrap();

            // TODO RL Early return/panic/exit if this condition not met
            if let Ok(lines) = read_lines(filename) {
                println!("\n{} {}", "filename:".cyan(), filename.bright_green());
                // Consumes the iterator, returns an (Optional) String
                for line in lines.map_while(Result::ok) {
                    if line.trim() == "---" {
                        yaml_delimiter_count += 1;
                    }

                    if yaml_delimiter_count == 1 && line.trim() != "---" && yaml_delimiter_count < 2
                    {
                        yaml_content.push_str(&format!("{}{}", &line, "\n"));
                    }

                    if yaml_delimiter_count == 2 && line.trim() != "---" {
                        markdown_content.push_str(&format!("{}{}", &line, "\n"));
                    }
                }
            }
            yaml_delimiter_count = 0;

            let yaml: Value = serde_yaml::from_str(&yaml_content).unwrap();
            // println!("yaml Value {:#?}", yaml);
            // Convert Front Matter YAML to a BTreeMap
            let yaml_btreemap: BTreeMap<String, Value> = yaml_mapping_to_btreemap(&yaml).unwrap();
            // println!("{}\n{:#?}", "yaml_btreemap".yellow(), yaml_btreemap);

            // Insert Font Matter YAML into markdown (if applicable)
            // TODO RL Add some sort of boolean check
            let merged_markdown_yaml =
                merge_markdown_yaml(yaml_btreemap.clone(), &markdown_content);

            // Convert Markdown content to HTML
            // markdown:: comes from the markdown crate
            let html: String = markdown::to_html(&merged_markdown_yaml.to_owned());
            println!("{}\n{:#?}", "HTML".cyan(), html);

            // Remove the markdown, md, file extension
            // TODO RL Just extract the file name, no path
            let filename_path = filename.trim_end_matches(".md");
            let extracted_filename = extract_to_end_string(filename_path, '/');

            println!("{} {}", "filename_path: ".cyan(), filename_path);
            println!(
                "{} {}",
                "extracted_filename: ".cyan(),
                extracted_filename.unwrap()
            );
            let _ = build_pdf(
                html,
                extracted_filename.unwrap(),
                yaml_btreemap,
                self.output_directory.to_path_buf(),
                <std::option::Option<std::collections::BTreeMap<std::string::String, std::string::String>> as Clone>::clone(&self.pdf_document_entries).unwrap(),
                self.pdf_version.clone(),
            );

            file += 1;
            // Reset yaml and markdown content ready for the next file
            yaml_content = String::default();
            markdown_content = String::default();
        }
    }

    pub fn set_doc_info_entry(&mut self, entry: PDFDocInfoEntry) {
        // TODO RL Check PDF specs section re capitalised metadata field name
        let local_doc_info_entry =
            entry.doc_info_entry[0..1].to_uppercase() + &entry.doc_info_entry[1..];
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
