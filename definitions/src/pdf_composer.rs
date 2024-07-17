// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{collections::BTreeMap, fmt, path::PathBuf};

use crate::fonts::FontsStandard;
use crate::page_properties::{PageMargins, PaperOrientation, PaperSize};
use crate::pdf_version::PDFVersion;

// pub use definitions::consts::PACKAGE_NAME;
// pub use definitions::fonts::FontsStandard;
// pub use definitions::page_properties::{PageMargins, PaperOrientation, PaperSize};
// pub use definitions::pdf_version::PDFVersion;

/// PDFComposer struct represents a tool for composing PDF documents from multiple source files.
pub struct PDFComposerStruct {
    /// Vector containing paths to the source files used for composing the PDF document.
    pub fmy_source_files: Vec<PathBuf>,
    /// Path to the directory where the composed PDF document will be saved.
    pub output_directory: PathBuf,
    /// Specifies the version of the PDF format to be used.
    pub pdf_version: PDFVersion,
    /// Optional mapping of document entries, where the key represents the entry name and the value represents the content.
    pub pdf_document_entries: Option<BTreeMap<String, String>>,
    /// Specifies the paper size for the PDF document.
    pub paper_size: PaperSize,
    /// Specifies the orientation of the page.
    pub orientation: PaperOrientation,
    /// Set the margins for the pages
    pub margins: PageMargins,
    /// Set the for the PDF document
    pub font: FontsStandard,
}

impl fmt::Debug for PDFComposerStruct {
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

// trait for PDFComposer is defined in the base workspace because of cross-crate traits and type rules
