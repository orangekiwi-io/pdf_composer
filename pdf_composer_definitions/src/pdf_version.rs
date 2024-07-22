use std::fmt;

/// Enum to represent different versions of the PDF specification.
/// See <https://pdfa.org/resource/pdf-specification-archive/> for more information on the PDF specifications
#[derive(Clone, Copy, Debug)]
pub enum PDFVersion {
    /// Represents PDF version 1.7.
    V1_7,
    /// Represents PDF version 2.0.
    V2_0,
}

/// Implements fmt:Display trait (warning in rust 1.78.0) for PDFVersion, converting enum variants to their corresponding String representations.
impl fmt::Display for PDFVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PDFVersion::V1_7 => write!(f, "1.7"),
            PDFVersion::V2_0 => write!(f, "2.0"),
        }
    }
}
