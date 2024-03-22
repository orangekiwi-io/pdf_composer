use colored::Colorize;
use headless_chrome::Browser;
use lopdf::{Document, Object as LopdfObject, StringFormat};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::utils::extract_to_end_string;
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
/// use your_crate_name::generate_pdf;
///
/// let generated_html = "<html><body><h1>Hello, world!</h1></body></html>".to_string();
/// let filename = "example";
/// let result = generate_pdf(generated_html, filename);
/// assert!(result.is_ok());
/// ```
pub fn generate_pdf(
    generated_html: String,
    filename_path: &str,
    yaml_btreemap: BTreeMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut string_values_btreemap: BTreeMap<String, String> = BTreeMap::new();
    for (key, value) in yaml_btreemap {
        if let Value::String(string_value) = value {
            string_values_btreemap.insert(key, string_value);
        }
    }

    let browser = Browser::default()?; // Start a new headless Chrome browser instance
    let tab = browser.new_tab()?; // Open a new tab

    let mut html = String::new();
    // Encode the HTML content to URL-safe format
    // url_escape:: comes from the url_escape crate
    url_escape::encode_query_to_string(generated_html, &mut html);

    // TODO RL Allow path to be set by the user, keeping "pdfs" as a fallback/default location
    let output_directory = "pdfs";
    fs::create_dir_all(output_directory)?;
    let extracted_filename = extract_to_end_string(filename_path, '/');
    let mut pdf_file = extracted_filename.unwrap().to_string();
    pdf_file.push_str(".pdf");

    let pdf_file_path = Path::new(output_directory).join(pdf_file);

    // Navigate the tab to the HTML content.
    // In this case, the page is a data stream
    tab.navigate_to(
        format!("data:text/html;charset=utf-8,{}", html).as_str(),
    )?;

    // Convert the page to PDF format
    let pdf = tab.print_to_pdf(None)?;
    // println!("{:?}", String::from_utf8_lossy(&pdf));

    // Create a new PDF document
    let mut doc: Document = Document::load_mem(&pdf)?;
    doc.version = "1.7".to_string();

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

                    // Print out all Key/Value pairs
                    // println!(
                    //     " {} {} :{} {:#?}",
                    //     "key".cyan(),
                    //     ascii_key,
                    //     "value".yellow(),
                    //     value
                    // );

                    // Iterate over the key-value pairs in the dictionary
                    // Check if the key is "Creator"
                    if ascii_key == "Creator" {
                        // Update the value associated with the key
                        let ascii_string =
                            string_values_btreemap.get("generator").unwrap();
                        let ascii_bytes: Vec<u8> =
                            ascii_string.as_bytes().to_vec();
                        *value = lopdf::Object::String(
                            ascii_bytes,
                            StringFormat::Literal,
                        );
                        // Set creator_found to true
                        creator_found = true;
                    }
                    if ascii_key == "Producer" {
                        // Update the value associated with the key
                        let ascii_string = "OrangeKiwi using lopdf";
                        let ascii_bytes: Vec<u8> =
                            ascii_string.as_bytes().to_vec();
                        *value = lopdf::Object::String(
                            ascii_bytes,
                            StringFormat::Literal,
                        );
                    }
                }
                // If Creator key was found, add/update various PDF properties/meta-data
                if creator_found {
                    // Insert or update Title key
                    let (key, value) = populate_dictionary(
                        "Title".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Author key
                    let (key, value) = populate_dictionary(
                        "Author".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Subject key
                    let (key, value) = populate_dictionary(
                        "Subject".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Keywords key
                    let (key, value) = populate_dictionary(
                        "Keywords".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Language key
                    let (key, value) = populate_dictionary(
                        "Language".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Permalink key
                    let (key, value) = populate_dictionary(
                        "Permalink".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
                    // Insert or update Site_components key
                    let (key, value) = populate_dictionary(
                        "Site_components".to_string(),
                        string_values_btreemap.clone(),
                    );
                    dictionary.set(key, value);
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

    doc.save(pdf_file_path)
        .expect("Failed to save modified PDF document");

    Ok(())
}

fn populate_dictionary(
    yaml_entry: String,
    string_values_btreemap: BTreeMap<String, String>,
) -> (Vec<u8>, LopdfObject) {
    println!("{}: {}", "Populate PDF dictionary key".yellow(), yaml_entry.cyan());
    let yaml_entry: String = match yaml_entry.as_str() {
        "Subject" => "Description".to_string(),
        _ => yaml_entry
    };
    let key = yaml_entry.as_bytes().to_vec();
    let value_string =
        string_values_btreemap.get(&yaml_entry.to_lowercase()).unwrap();
    let value_as_bytes: Vec<u8> = value_string.as_bytes().to_vec();

    (key, LopdfObject::String(value_as_bytes, StringFormat::Literal))
}
