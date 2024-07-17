// Copyright © 2024 PDF Composer (pdf_composer). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The 'definitions' crate for defining the structs, consts, enums etc for PDF Composer.
//!
//! This crate provides the core structures and definitions used throughout the PDF Composer crate.
//! It includes modules for handling constants, fonts, output directories, page properties,
//! PDF composition, document entries, and valid PDF versions.

/// Module containing constant values used throughout PDF Composer
pub mod consts;

/// Module handling font-related functionality
pub mod fonts;

/// Module handling the output directory (as a str or path)
pub mod output_directory;

/// Module defining and handling page properties (such as size and orientation)
pub mod page_properties;

/// Module defining the core PDF Composer struct
pub mod pdf_composer;

/// Module defining the structure for PDF document entries (key/value pairs)
pub mod pdf_doc_entry;

/// Module to re-export the PDF version enum
pub mod pdf_version;
