use std::fmt;

/// Struct representing an entry in the PDF document information.
/// Used to set things up ready for lopdf
pub struct PDFDocInfoEntry<'a> {
    /// The name of the document information entry.
    pub doc_info_entry: &'a str,
    /// The corresponding YAML entry associated with the document information.
    pub yaml_entry: &'a str,
}

impl<'a> fmt::Debug for PDFDocInfoEntry<'a> {
    /// Implements the Debug trait for the PDFDocInfoEntry struct, allowing it to be formatted for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PDFDocInfoEntry")
            .field("doc_info_entry", &self.doc_info_entry)
            .field("yaml_entry", &self.yaml_entry)
            .finish()
    }
}
