// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// The `build_pdf` module contains functions for generating PDF files.
pub(crate) mod build_pdf;
/// Re-exports the `build_pdf` function for public use.
pub(crate) use build_pdf::build_pdf;

/// The `page_properties` module contains enums, traits, structs and functions related to page properties (dimensions, margins etc).
mod page_properties;
pub(crate) use page_properties::PageMargins;
/// Re-exports the `PaperSize` and `PaperOrientation` enums for public use.
pub use page_properties::{PaperOrientation, PaperSize};

/// The `fonts` module contains enums, traits, structs and functions related to font for PDF documents
mod fonts;
/// Re-exports the `FontsStandard` enum for public use.
pub use fonts::FontsStandard;
