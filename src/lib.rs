// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/orangekiwi-io/pdf_composer/main/assets/PDFComposer.png"
)]
#![doc = include_str!("../README.md")]
// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

use colored::Colorize;
use rayon::prelude::*;
use serde_yaml::Value;
use std::{collections::BTreeMap, fmt, fs, path::PathBuf, process};

mod utils;
use utils::{merge_markdown_yaml, read_lines, yaml_mapping_to_btreemap};

/// The 'core' module containing the main functions for PDF Composer
pub mod core;
use core::{build_pdf, PageMargins};
pub use core::{FontsStandard, PaperOrientation, PaperSize};

use crate::core::build_pdf::PDFBuilder;

/// CONST for a tick/check mark character plus a space character
pub const CHECK_MARK: &str = "\u{2713} ";
/// CONST for a cross character plus a space character
pub const CROSS_MARK: &str = "\u{2717} ";

const DEFAULT_MARGIN: f64 = 10.0;
const MM_TO_INCH: f64 = 25.4;

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
    /// Specifies the paper size for the PDF document.
    paper_size: PaperSize,
    /// Specifies the orientation of the page.
    orientation: PaperOrientation,
    /// Set the margins for the pages
    margins: PageMargins,
    /// Set the for the PDF document
    font: FontsStandard,
}

impl fmt::Debug for PDFComposer {
    /// Implements the Debug trait for the PDFComposer struct, allowing it to be formatted for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFComposer")
            .field("fmy_source_files", &self.fmy_source_files)
            .field("output_directory", &self.output_directory)
            .field("pdf_version", &self.pdf_version)
            .field("pdf_document_entries", &self.pdf_document_entries)
            .field("paper_size", &self.paper_size)
            .field("orientation", &self.orientation)
            .field("margins", &&self.margins)
            .field("font", &&self.font)
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
            paper_size: PaperSize::A4,
            orientation: PaperOrientation::Portrait,
            margins: [DEFAULT_MARGIN / MM_TO_INCH; 4],
            font: FontsStandard::Helvetica,
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
    /// my_pdf_doc.set_pdf_version(PDFVersion::V1_7);
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

    /// Sets the paper size for the PDF documents.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the paper size to A5
    /// my_pdf_doc.set_paper_size(PaperSize::A5);
    /// ```
    pub fn set_paper_size(&mut self, paper_size: PaperSize) {
        self.paper_size = paper_size;
    }

    /// Sets the page orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the orientation to Landscape
    /// my_pdf_doc.set_orientation(PaperOrientation::Landscape);
    /// ```
    pub fn set_orientation(&mut self, orientation: PaperOrientation) {
        self.orientation = orientation;
    }

    /// Sets the font for the PDF.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the font to Times Roman
    /// my_pdf_doc.set_font(FontsStandard::TimesRoman);
    /// ```
    pub fn set_font(&mut self, font: FontsStandard) {
        self.font = font;
    }

    /// Sets the page margins.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdf_composer::PDFComposer;
    ///
    /// // Create a new PDF generator instance
    /// let mut my_pdf_doc = PDFComposer::new();
    ///
    /// // Set the page margins to 20mm
    /// my_pdf_doc.set_margins("20");
    /// ```
    pub fn set_margins(&mut self, margins: &str) {
        // println!("{} {}", "margins:".cyan(), margins);
        // Trim (remove) white space from both ends of the margins string
        let mut margins_vector: Vec<&str> = margins.trim().split(' ').collect();
        // Remove all empty elements in the margins vector
        margins_vector.retain(|ele| !ele.is_empty());
        // println!(
        //     "{} {:?}",
        //     "margins_vector:".cyan(),
        //     margins_vector.to_owned()
        // );

        // Check to see if there are any non-integer entries for margin values
        // If there are, then set any_letters_found to true and set all margins to default size
        let any_letters_found = margins_vector
            .iter()
            .any(|&ele| ele.parse::<u32>().is_err());

        if any_letters_found {
            self.margins = [DEFAULT_MARGIN / MM_TO_INCH; 4];
            let troublesome_margins: String = margins_vector.join(", ");
            let margin_error_message = "".to_owned()
                + &CROSS_MARK.red().to_string()
                + &"Something wrong with the margin values provided "
                    .red()
                    .to_string()
                + &"[".yellow().to_string()
                + &troublesome_margins.yellow().to_string()
                + &"]".yellow().to_string()
                + "\nUsing the default value of "
                + &DEFAULT_MARGIN.to_string()
                + "mm for the margins.\n";
            eprintln!("{}", margin_error_message);
        } else {
            self.margins = match margins_vector.len() {
                1 => {
                    if margins_vector[0].is_empty() {
                        [DEFAULT_MARGIN / MM_TO_INCH; 4]
                    } else {
                        [f64::from(margins_vector[0].parse::<u32>().unwrap()) / MM_TO_INCH; 4]
                    }
                }
                2 => {
                    let top_bottom =
                        f64::from(margins_vector[0].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let left_right =
                        f64::from(margins_vector[1].parse::<u32>().unwrap()) / MM_TO_INCH;
                    [top_bottom, left_right, top_bottom, left_right]
                }
                3 => {
                    let top = f64::from(margins_vector[0].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let left_right =
                        f64::from(margins_vector[1].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let bottom = f64::from(margins_vector[2].parse::<u32>().unwrap()) / MM_TO_INCH;
                    [top, left_right, bottom, left_right]
                }
                4 => {
                    let top = f64::from(margins_vector[0].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let right = f64::from(margins_vector[1].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let bottom = f64::from(margins_vector[2].parse::<u32>().unwrap()) / MM_TO_INCH;
                    let left = f64::from(margins_vector[3].parse::<u32>().unwrap()) / MM_TO_INCH;
                    [top, right, bottom, left]
                }
                _ => [DEFAULT_MARGIN / MM_TO_INCH; 4],
            }
        };

        // println!("{:#?}", self.margins);
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
                        filename.cyan(),
                        "Reading...".green()
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
                        println!("File {} is not a valid yaml file", filename.red());
                        break 'file_found;
                    } else {
                        println!(
                            "{}. {}",
                            filename.cyan(),
                            "Processing...".green()
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

                    let instance_data = PDFBuilder {
                        source_file: filename.to_string(),
                        output_directory: self.output_directory.to_path_buf(),
                        pdf_version: self.pdf_version,
                        paper_size: self.paper_size,
                        orientation: self.orientation,
                        margins: self.margins,
                        font: self.font,
                    };

                    // Build the PDF document.
                    let _ = build_pdf(
                        html,
                        // filename.to_string(),
                        yaml_btreemap,
                        // self.output_directory.to_path_buf(),
                        <std::option::Option<
                            std::collections::BTreeMap<std::string::String, std::string::String>,
                        > as Clone>::clone(&self.pdf_document_entries)
                        .unwrap(),
                        // self.pdf_version,
                        // self.paper_size,
                        // self.orientation,
                        // self.margins,
                        // self.font,
                        instance_data,
                    );
                }
                Err(_) => {
                    // File not found, print error message.
                    println!("File {} not found.", filename.red());
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
