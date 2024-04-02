use colored::Colorize;
use lopdf::{Document, Object as LopdfObject, StringFormat};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use crate::utils::extract_to_end_string;
use async_std::task;
use chromiumoxide::{cdp::browser_protocol::page::PrintToPdfParams, Browser, BrowserConfig};
use futures::StreamExt;

const CHECK_MARK: &str = "\u{2713} ";
const CROSS_MARK: &str = "\u{2717} ";
/// Generates a PDF from HTML content using headless Chrome.
///
/// # Arguments
///
/// * `generated_html` - The HTML content to convert to PDF.
/// * `filename` - The name of the PDF file to generate.
///
/// # Errors
///
/// Returns a boxed error if there is an issue with the headless Chrome browser, navigation,
/// capturing screenshot, printing to PDF, or writing the PDF file.
///
/// # Examples
///
/// ```
/// use your_crate_name::build_pdf;
///
/// let generated_html = "<html><body><h1>Hello, world!</h1></body></html>".to_string();
/// let filename = "example";
/// let result = build_pdf(generated_html, filename);
/// assert!(result.is_ok());
/// ```
pub fn build_pdf(
    generated_html: String,
    source_file: String,
    // filename: &str,
    yaml_btreemap: BTreeMap<String, Value>,
    output_directory: PathBuf,
    dictionary_entries: BTreeMap<String, String>,
    pdf_version: String,
) -> Result<(), Box<dyn std::error::Error>> {
    task::block_on(async {
        // println!("{}: {:#?}", "dictionary_entries".cyan(), &dictionary_entries);
        // Remove the markdown, md, file extension
        let filename_path = source_file.trim_end_matches(".md");
        // Extract only the file name
        let extracted_filename = extract_to_end_string(filename_path, '/');

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

        let title_string = yaml_btreemap.get("title").unwrap().as_str().unwrap();
        let mut html_string = String::new();
        let html_before_string = format!("<html><head><title>{}</title><head><body>", title_string);
        let html_after_string = "</body></html>";
        // Encode the HTML content to URL-safe format
        // url_escape:: comes from the url_escape crate
        url_escape::encode_query_to_string(generated_html, &mut html_string);

        create_dir_all(&output_directory)?;
        let mut pdf_file = extracted_filename.unwrap().to_string();
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
        let pdf = page.pdf(PrintToPdfParams::default()).await?;

        // Create a new PDF document
        let mut doc: Document = Document::load_mem(&pdf)?;
        doc.version = pdf_version;

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
                            let default_creator = "PDFComposer".to_string();
                            let ascii_string = string_values_btreemap
                                .get("generator")
                                .unwrap_or(&default_creator);
                            let ascii_bytes: Vec<u8> = ascii_string.as_bytes().to_vec();
                            *value = lopdf::Object::String(ascii_bytes, StringFormat::Literal);
                            // Set creator_found to true
                            creator_found = true;
                        }
                        if ascii_key == "Producer" {
                            // Update the value associated with the key
                            let ascii_string = "OrangeKiwi using Chromiumoxide and lopdf";
                            let ascii_bytes: Vec<u8> = ascii_string.as_bytes().to_vec();
                            *value = lopdf::Object::String(ascii_bytes, StringFormat::Literal);
                        }
                    }
                    // If Creator key was found, add/update various PDF properties/metadata
                    if creator_found {
                        // Loop through properties set by user
                        for entry in &dictionary_entries {
                            // println!("----------> {}: {}", entry.0.cyan(), entry.1.green());
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
                    source_file.bright_green(),
                    pdf_file_path_as_string.yellow()
                );
                println!("{}", "PDF document metadata properties".yellow());

                for entry in &dictionary_entries {
                    let entry_exists = check_entry_exists(entry.1.to_string(), &string_values_btreemap);

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

fn populate_dictionary(
    yaml_entry: String,
    string_values_btreemap: BTreeMap<String, String>,
) -> (Vec<u8>, LopdfObject) {

    let key = yaml_entry.as_bytes().to_vec();
    let value_string = string_values_btreemap
        .get(&yaml_entry.to_lowercase())
        .unwrap();

    let value_as_bytes: Vec<u8> = value_string.as_bytes().to_vec();

    (
        key,
        LopdfObject::String(value_as_bytes, StringFormat::Literal),
    )
}

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

fn check_entry_exists(entry: String, btree: &BTreeMap<String, String>) -> bool {
    let mut entry_exists = false;

    for value in btree.keys() {
        if *value == entry {
            entry_exists = true;
        }
    }

    entry_exists
}
