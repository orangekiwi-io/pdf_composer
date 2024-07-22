// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/orangekiwi-io/pdf_composer/main/assets/PDFComposer.png"
)]
#![doc = include_str!("../README.md")]
// Crate configuration
#![crate_name = "pdf_composer"]
#![crate_type = "lib"]

pub use pdf_composer_base::PDFComposer;
pub use pdf_composer_definitions::consts::PACKAGE_NAME;
pub use pdf_composer_definitions::fonts::FontsStandard;
pub use pdf_composer_definitions::page_properties::{PageMargins, PaperOrientation, PaperSize};
pub use pdf_composer_definitions::pdf_composer::PDFComposerStruct;
pub use pdf_composer_definitions::pdf_doc_entry::PDFDocInfoEntry;
pub use pdf_composer_definitions::pdf_version::PDFVersion;
