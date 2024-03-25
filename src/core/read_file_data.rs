use colored::Colorize;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::core::generate_pdf;
use crate::utils::{merge_markdown_yaml, read_lines, yaml_mapping_to_btreemap};

/// Reads data from Markdown files, extracts YAML front matter, and generates PDF files.
///
/// # Arguments
///
/// * `files` - A vector of file paths to Markdown files containing Front Mattter YAML.
///
/// # Examples
///
/// ```
/// use your_crate_name::read_file_data;
///
/// let files = vec!["./source_files/404.md"];
/// read_file_data(files);
/// ```
pub(crate) fn read_file_data(files: &[PathBuf], output_directory: &Path, pdf_document_entries: Option<BTreeMap<String, String>>) {
    let mut file = 0;
    let mut yaml_delimiter_count = 0;
    let mut yaml_content: String = String::default();
    let mut markdown_content: String = String::default();
    println!("Files to process: {}", files.len());

    while file < files.len() {
        let filename = &<std::path::PathBuf as Clone>::clone(&files[file]).into_os_string().into_string().unwrap();
        println!("filename: {}", filename);

        // let filename_as_string = filename.into_os_string().into_string().unwrap();
        if let Ok(lines) = read_lines(filename) {
            println!("{}", filename.bright_yellow());
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                if line.trim() == "---" {
                    yaml_delimiter_count += 1;
                }

                if yaml_delimiter_count == 1 && line.trim() != "---" && yaml_delimiter_count < 2 {
                    yaml_content.push_str(&format!("{}{}", &line, "\n"));
                }

                if yaml_delimiter_count == 2 && line.trim() != "---" {
                    markdown_content.push_str(&format!("{}{}", &line, "\n"));
                }
            }
        }
        yaml_delimiter_count = 0;

        let yaml: Value = serde_yaml::from_str(&yaml_content).unwrap();
        println!("yaml Value {:#?}", yaml);
        // Convert Front Matter YAML to a BTreeMap
        let yaml_btreemap: BTreeMap<String, Value> = yaml_mapping_to_btreemap(&yaml).unwrap();

        // Insert Font Matter YAML into markdown (if applicable)
        // TODO RL Add some sort of boolean check
        let merged_markdown_yaml = merge_markdown_yaml(yaml_btreemap.clone(), &markdown_content);

        // Convert Markdown content to HTML
        // markdown:: comes from the markdown crate
        let html: String = markdown::to_html(&merged_markdown_yaml.to_owned());
        println!("{} {:#?}", "HTML".cyan(), html);

        // Remove the markdown, md, file extension
        let filename_path = filename.trim_end_matches(".md");
        println!("{} {:#?}", "filename_path".cyan(), filename_path);
        let _ = generate_pdf(html, filename_path, yaml_btreemap, output_directory.to_path_buf());

        file += 1;
        // Reset yaml and markdown content ready for the next file
        yaml_content = String::default();
        markdown_content = String::default();
    }
}
