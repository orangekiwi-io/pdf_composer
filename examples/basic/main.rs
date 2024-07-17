// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # PDF Composer Examples
//!
//! This file is an example usage of the PDF Composer crate.
//!
use pdf_composer::{
    FontsStandard, PDFComposer, PDFComposerStruct, PDFDocInfoEntry, PDFVersion, PaperOrientation,
    PaperSize,
};
use std::path::PathBuf;

fn main() {
    println!("Basic example");

    // Create a new PDFComposer instance
    let mut bob: PDFComposerStruct = PDFComposer::new();

    // Add some paths. Relative paths
    let paths = vec![
        PathBuf::from("examples/basic/sample_mds/sample_file_01.md"),
        PathBuf::from("examples/basic/sample_mds/sample_file_02.md"),
        PathBuf::from("examples/basic/sample_mds/file_not_found.md"),
        PathBuf::from("examples/basic/sample_mds/untitled.txt"),
    ];
    bob.add_source_files(paths);

    // PDF version (not the version of the document, but the Adobe (formerly) PDF format version)
    bob.set_pdf_version(PDFVersion::V2_0);

    // Output directory for the generated PDFs
    bob.set_output_directory("examples/basic/output_pdfs_a6");

    // Defaults to PaperSize::A4 if no PaperSize provided
    bob.set_paper_size(PaperSize::A6);

    // Set the paper orientation. Default to PaperOrientation::Portrait
    bob.set_orientation(PaperOrientation::Landscape);

    // Set the page margins (mm). Defaults to 10 (mm)
    bob.set_margins("20");

    // Set font. Defaults to FontsStandard::Helvetica
    bob.set_font(FontsStandard::TimesRoman);

    // Metadata for the PDFs
    // Title property set via the HTML template <title> tag
    let author_entry = PDFDocInfoEntry {
        doc_info_entry: "Author",
        yaml_entry: "author",
    };
    let keywords_entry = PDFDocInfoEntry {
        doc_info_entry: "Keywords",
        yaml_entry: "keywords",
    };
    let subject_entry = PDFDocInfoEntry {
        doc_info_entry: "Subject",
        yaml_entry: "description",
    };
    let language_entry = PDFDocInfoEntry {
        doc_info_entry: "Language",
        yaml_entry: "language",
    };
    // This entry will only appear in the generated PDF for sample_file_01
    let random_entry = PDFDocInfoEntry {
        doc_info_entry: "Random",
        yaml_entry: "random",
    };
    bob.set_doc_info_entry(author_entry);
    bob.set_doc_info_entry(keywords_entry);
    bob.set_doc_info_entry(random_entry);
    bob.set_doc_info_entry(subject_entry);
    bob.set_doc_info_entry(language_entry);

    // Generate the PDFs
    bob.generate_pdfs();
}
