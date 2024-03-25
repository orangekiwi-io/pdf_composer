// /// The `source_files` module contains functions for working with source files.
// mod sources_files;
// pub use sources_files::get_source_front_matter_files;

// use std::{collections::BTreeMap, path::{Path, PathBuf}};

/// The `generate_pdf` module contains functions for generating PDF files.
mod generate_pdf;
pub use self::generate_pdf::generate_pdf;

// The `read_file_data` module contains functions for reading files.
// mod read_file_data;
// use read_file_data::read_file_data;
// pub fn read_files(files: &[PathBuf], output_directory: &Path, pdf_document_entries: &Option<BTreeMap<String, String>>) {
//     read_file_data(files, output_directory, pdf_document_entries.clone());
// }
