// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use colored::Colorize;
use lopdf::{Document, Object as LopdfObject, StringFormat};
use serde_yml::Value;
use std::collections::BTreeMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use crate::utils::extract_to_end_string;
use async_std::task;
use chromiumoxide::{cdp::browser_protocol::page::PrintToPdfParams, Browser, BrowserConfig};
use pdf_composer_definitions::consts::{CHECK_MARK, CROSS_MARK, PACKAGE_NAME};
use pdf_composer_definitions::fonts::{FontsStandard, GetCssName};
use pdf_composer_definitions::page_properties::{
    PageMargins, PaperOrientation, PaperSize, ToDimensions,
};
use pdf_composer_definitions::pdf_version::PDFVersion;

use futures::StreamExt;

/// This function generates a PDF document from a given HTML string, source file and YAML data.
/// It also all updated dictionary entries, PDF version, paper size, paper orientation sets margins and the font before writing PDFs to the output directory.
///
/// # Arguments
///
/// * `generated_html` - A `String` containing the HTML content to be converted to PDF.
/// * `yaml_btreemap` - A `BTreeMap<String, Value>` containing the YAML data.
/// * `dictionary_entries` - A `BTreeMap<String, String>` containing key-value pairs to be added or updated in the PDF document's metadata dictionary.
/// * `instance_data` - An object containing the smaller data about the PDF (orientation, source_file, output_directory, pdf_version, paper_size, margins, font).
///
/// # Returns
///
/// * `Ok(())` if the PDF document was successfully generated and saved.
/// * `Err(e)` if an error occurred during the process, where `e` is a `Box<dyn std::error::Error>` containing the error information.
///
/// # Remarks
///
/// This function performs the following tasks:
///
/// 1. Launches a Headless Chromium browser instance using the `Browser::launch` method.
/// 2. Constructs the HTML content by combining the generated HTML with a basic HTML structure and encoding it for URL safety.
/// 3. Creates a new browser page and navigates to the HTML content.
/// 4. Converts the page content to PDF format using the `page.pdf` method.
/// 5. Creates a new `Document` object from the PDF data using the `Document::load_mem` method.
/// 6. Updates the PDF document version based on the provided `pdf_version`.
/// 7. Sets the paper size `paper_size`
/// 8. Sets the paper margins `margins`
/// 9. Sets the PDF font `font`
/// 10. Set the orientation for the paper `orientation`
/// 11. Iterates over the objects in the PDF document and updates the "Creator" and "Producer" metadata entries, if present.
/// 12. If the "Creator" metadata entry is found, adds or updates the PDF document's metadata properties based on the `dictionary_entries`.
/// 13. Saves the modified PDF document to the specified output directory with a filename derived from the source file.
/// 14. Displays a success message with the path to the generated PDF file and the updated metadata properties.
///
/// The function handles cases where the PDF file is already open by another process and prints an error message if an error occurs during the process.
pub fn build_pdf(
    generated_html: String,
    yaml_btreemap: BTreeMap<String, Value>,
    dictionary_entries: BTreeMap<String, String>,
    instance_data: PDFBuilder,
) -> Result<(), Box<dyn std::error::Error>> {
    // Destructure instance_data (PDFBuilder struct)
    let PDFBuilder {
        orientation,
        source_file,
        output_directory,
        pdf_version,
        paper_size,
        margins,
        font,
    } = instance_data;

    // Set page size for all PDF documents based on orientation.
    let (page_width, page_height) = match orientation {
        PaperOrientation::Landscape => (paper_size.to_dimensions().1, paper_size.to_dimensions().0),
        PaperOrientation::Portrait => paper_size.to_dimensions(),
    };

    task::block_on(async {
        // Remove the markdown, md, file extension
        let filename_path = source_file.trim_end_matches(".md");
        // Extract only the file name
        let extracted_filename = extract_to_end_string(filename_path);
        let extracted_filename_as_string = extracted_filename.unwrap().to_string();

        let mut string_values_btreemap: BTreeMap<String, String> = BTreeMap::new();
        for (key, value) in yaml_btreemap.clone() {
            if let Value::String(string_value) = value {
                string_values_btreemap.insert(key, string_value);
            }
        }
        let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;

        let _handle = async_std::task::spawn(async move {
            loop {
                let _event = handler.next().await.unwrap();
            }
        });

        // TODO RL Template this? External file?
        // Set CSS @media print media query and @page property for pages
        let mut css_page = String::from("<style>\n@media print {\n ");
        let (css_font_name, css_font_weight, css_font_style) = font.get_css_name();
        let css_font = format!(
            "body {{ font-family: {}; font-weight: {}; font-style: {} }}\n\n",
            css_font_name, css_font_weight, css_font_style
        );
        let css_at_page = format!("@page {{\nsize: {}in {}in;\n}}", page_width, page_height);
        css_page.push_str(&css_font);
        css_page.push_str(&css_at_page);
        css_page.push_str("\n}\n</style>");

        // Set the title String to either the yaml 'title' entry,
        // or (if there is no 'title' entry), the filename of the source file in question
        let title_string = yaml_btreemap
            .get("title")
            .and_then(|value| value.as_str())
            .unwrap_or(&extracted_filename_as_string);
        let mut html_string = String::new();
        let html_before_string = format!(
            "<html><head><title>{}</title>{}</head><body>",
            title_string, css_page
        );
        let html_after_string = "</body></html>";

        // Encode the HTML content to URL-safe format
        // url_escape:: comes from the url_escape crate
        url_escape::encode_query_to_string(generated_html, &mut html_string);

        let mut pdf_file = extracted_filename_as_string;
        pdf_file.push_str(".pdf");

        let pdf_file_path = Path::new(&output_directory).join(pdf_file);
        let pdf_file_path_as_string = pdf_file_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();

        // Navigate the tab to the HTML content.
        // In this case, the page is a data stream
        let page = browser
            .new_page(
                format!(
                    "data:text/html;charset=utf-8,{}{}{}",
                    html_before_string, html_string, html_after_string
                )
                .as_str(),
            )
            .await?;
        let _html = page.wait_for_navigation().await?.content().await?;

        // Convert the page to PDF format
        let paper_settings = PrintToPdfParams {
            // landscape: todo!(),
            // display_header_footer: todo!(),
            // print_background: todo!(),
            // scale: todo!(),
            paper_width: Some(page_width),
            paper_height: Some(page_height),
            margin_top: Some(margins[0]),
            margin_right: Some(margins[1]),
            margin_bottom: Some(margins[2]),
            margin_left: Some(margins[3]),
            // page_ranges: todo!(),
            // header_template: todo!(),
            // footer_template: todo!(),
            prefer_css_page_size: Some(true),
            // transfer_mode: todo!(),
            ..Default::default()
        };

        // let pdf = page.pdf(PrintToPdfParams::default()).await?;
        let pdf = page.pdf(paper_settings).await?;

        // Create a new PDF document
        let mut doc: Document = Document::load_mem(&pdf)?;
        doc.version = pdf_version.to_string();

        doc.compress();
        create_dir_all(pdf_file_path.parent().unwrap())?;
        doc.save(pdf_file_path.clone()).unwrap();

        #[allow(unused_variables)]
        let mut object_count: i32 = 0;
        // Iterate over the objects in the PDF document and count them
        for object_element in &mut doc.objects {
            let (_key, object) = object_element;
            match object {
                LopdfObject::Dictionary(dictionary) => {
                    // Variable to track if Creator key is present
                    let mut creator_found = false;

                    // Print out the dictionary entries
                    for (key, value) in dictionary.iter_mut() {
                        let ascii_key = String::from_utf8_lossy(key);

                        // Iterate over the key-value pairs in the dictionary
                        // Check if the key is "Creator"
                        if ascii_key == "Creator" {
                            // Update the value associated with the key
                            let default_creator = &PACKAGE_NAME.to_string();
                            let ascii_string = string_values_btreemap
                                .get("generator")
                                .unwrap_or(default_creator);
                            let ascii_bytes: Vec<u8> = ascii_string.as_bytes().to_vec();
                            *value = lopdf::Object::String(ascii_bytes, StringFormat::Literal);
                            // Set creator_found to true
                            creator_found = true;
                        }
                        if ascii_key == "Producer" {
                            // Update the value associated with the key
                            let ascii_string = PACKAGE_NAME;
                            let ascii_bytes: Vec<u8> = ascii_string.as_bytes().to_vec();
                            *value = lopdf::Object::String(ascii_bytes, StringFormat::Literal);
                        }
                    }
                    // If Creator key was found, add/update various PDF properties/metadata
                    if creator_found {
                        // Loop through properties set by user
                        for entry in &dictionary_entries {
                            let entry_exists =
                                check_entry_exists(entry.1.to_string(), &string_values_btreemap);

                            if entry_exists {
                                let (_key, value) = populate_dictionary(
                                    entry.1.to_string(),
                                    string_values_btreemap.clone(),
                                );
                                dictionary.set(entry.0.as_bytes().to_vec(), value);
                            }
                        }
                    }

                    object_count += 1;
                }
                LopdfObject::Stream(_) => {
                    // It's a stream object
                    object_count += 1;
                }
                _ => {
                    // It's some other type of object
                }
            }
        }

        let mut error_message = "\n".to_owned()
            + &CROSS_MARK.on_red().to_string()
            + &pdf_file_path_as_string.on_red().to_string()
            + "\n";
        error_message.push_str(
            "Failed to save modified PDF document."
                .red()
                .to_string()
                .as_str(),
        );

        match is_file_open(&pdf_file_path_as_string) {
            Ok(true) => println!("{} is open by another process.", &pdf_file_path_as_string),
            Ok(false) => {
                doc.save(pdf_file_path.clone()).unwrap();

                println!(
                    "\n{}{} → {}",
                    CHECK_MARK.to_string().green(),
                    source_file.green(),
                    pdf_file_path_as_string.yellow()
                );
                println!("{}", "PDF document metadata properties".yellow());

                for entry in &dictionary_entries {
                    let entry_exists =
                        check_entry_exists(entry.1.to_string(), &string_values_btreemap);

                    if entry_exists {
                        println!("* {}: {}", entry.0.cyan(), entry.1.green());
                    }
                }
            }
            Err(error) => println!("{} {}", error_message, error),
        }

        Ok(())
    })
}

/// PDFBuilder Struct for passing data into the build_pdf function
#[derive(Debug)]
pub struct PDFBuilder {
    /// `source_file` - A `String` representing the path to the source file (e.g., Markdown file) from which the HTML was generated.
    pub source_file: String,
    /// `output_directory` - A `PathBuf` representing the directory where the PDF file should be saved.
    pub output_directory: PathBuf,
    /// `pdf_version` - A `PDFVersion` enum value specifying the version of the PDF document.
    pub pdf_version: PDFVersion,
    /// `paper_size` - The paper size for the PDF document.
    pub paper_size: PaperSize,
    /// `orientation` - The orientation (landscape or portrait) of the paper for the PDF document.
    pub orientation: PaperOrientation,
    /// `margins` - Page margins.
    pub margins: PageMargins,
    /// `font` - The font to be used for the PDF document.
    pub font: FontsStandard,
}

/// This function populates a dictionary (BTreeMap) with a key-value pair.
/// The key is a byte vector representation of the `yaml_entry` string,
/// and the value is an `LopdfObject` containing a byte vector representation
/// of the corresponding string value from the `string_values_btreemap`.
///
/// # Arguments
///
/// * `yaml_entry` - A `String` representing the key for the dictionary entry.
/// * `string_values_btreemap` - A `BTreeMap<String, String>` containing the
///   string values to be used for populating the dictionary.
///
/// # Returns
///
/// A tuple containing:
///
/// * A `Vec<u8>` representing the key (byte vector of `yaml_entry`).
/// * An `LopdfObject` containing a byte vector representation of the
///   corresponding string value from `string_values_btreemap`, with a
///   `StringFormat::Literal` format.
///
/// # Panics
///
/// This function will panic if the `string_values_btreemap` does not contain
/// a value for the lowercase version of the `yaml_entry` key.
fn populate_dictionary(
    yaml_entry: String,
    string_values_btreemap: BTreeMap<String, String>,
) -> (Vec<u8>, LopdfObject) {
    // Convert the `yaml_entry` string to a byte vector to be used as the key
    let key = yaml_entry.as_bytes().to_vec();

    // Get the value from the `string_values_btreemap` corresponding to the
    // lowercase version of the `yaml_entry` key
    // This will panic if the key is not found in the BTreeMap
    let value_string = string_values_btreemap
        .get(&yaml_entry.to_lowercase())
        .unwrap();

    // Convert the value string to a byte vector
    let value_as_bytes: Vec<u8> = value_string.as_bytes().to_vec();

    // Return a tuple containing the key (byte vector of `yaml_entry`) and
    // an `LopdfObject` containing the byte vector representation of the value
    // string, with a `StringFormat::Literal` format
    (
        key,
        LopdfObject::String(value_as_bytes, StringFormat::Literal),
    )
}

/// This function checks if a file is open (exclusively locked by another process).
///
/// # Arguments
///
/// * `file_path` - A string slice (`&str`) representing the path to the file.
///
/// # Returns
///
/// * `Ok(true)` if the file is exclusively locked by another process.
/// * `Ok(false)` if the file is not exclusively locked and can be opened for writing.
/// * `Err(e)` if an error occurs while attempting to open the file for any reason
///   other than the file being exclusively locked (e.g., file not found, invalid
///   path, etc.).
///
/// # Remarks
///
/// This function attempts to open the specified file for writing using
/// `OpenOptions::new().write(true).open(file_path)`. If the file can be opened
/// successfully, it means that the file is not exclusively locked by another process,
/// so the function returns `Ok(false)`.
///
/// If the `open` operation fails with an `io::ErrorKind::PermissionDenied` error,
/// it indicates that the file is exclusively locked by another process, so the
/// function returns `Ok(true)`.
///
/// For any other error kind, the function propagates the error by returning `Err(e)`.
fn is_file_open(file_path: &str) -> Result<bool, io::Error> {
    match OpenOptions::new().write(true).open(file_path) {
        Ok(_) => {
            // The file was successfully opened, which means it's not exclusively locked by another process
            Ok(false)
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    // The file is exclusively locked by another process
                    Ok(true)
                }
                _ => Err(error),
            }
        }
    }
}

/// This function checks if a given entry (a key's value) exists in a `BTreeMap<String, String>`.
///
/// # Arguments
///
/// * `entry` - A `String` representing the entry (key's value) to search for in the `BTreeMap`.
/// * `btree` - A reference to the `BTreeMap<String, String>` in which to search for the entry.
///
/// # Returns
///
/// * `true` if the `entry` exists as a value in the `btree`.
/// * `false` if the `entry` does not exist as a value in the `btree`.
///
/// # Remarks
///
/// The function iterates over the keys of the `btree` and compares each key value with the `entry`.
/// If a match is found, the `entry_exists` flag is set to `true`, and the loop is terminated.
/// After the loop, the function returns the value of `entry_exists`.
fn check_entry_exists(entry: String, btree: &BTreeMap<String, String>) -> bool {
    // Initialize a mutable boolean flag `entry_exists` to false
    let mut entry_exists = false;

    // Iterate over the keys of the `btree`
    for (key, _value) in btree.iter() {
        // If the current key is equal to the `entry`
        if key == &entry {
            // Set `entry_exists` to true and break out of the loop
            entry_exists = true;
            break;
        }
    }

    // Return the final value of `entry_exists`
    entry_exists
}
