// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// The `build_pdf` module contains functions for generating PDF files.
mod build_pdf;
/// Re-exports the `build_pdf` function for public use.
pub use build_pdf::build_pdf;

/// The `page_properties` module contains enums, traits, structs and functions related to page properties (dimensions, margins etc).
mod page_properties;
/// Re-exports the `PaperSize` enum for public use.
pub use page_properties::{PaperSize, PaperOrientation};