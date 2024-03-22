// /// The `source_files` module contains functions for working with source files.
// mod sources_files;
// pub use sources_files::get_source_front_matter_files;

/// The `generate_pdf` module contains functions for generating PDF files.
mod generate_pdf;
pub use generate_pdf::generate_pdf;

/// The `read_file_data` module contains functions for reading files.
mod read_file_data;
pub use read_file_data::read_file_data;

/// The `extract_to_end_string` module contains a function to extract everything to the end of the string after selected delimiter.
mod extract_to_end_string;
pub use extract_to_end_string::extract_to_end_string;
