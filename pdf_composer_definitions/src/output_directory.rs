/// A trait representing an output directory.
pub trait OutputDirectory {
    /// Converts the reference to a path buffer.
    fn convert(&self) -> std::path::PathBuf;
}

/// Implement the `OutputDirectory` trait for `&str`.
impl OutputDirectory for &str {
    /// Converts the reference to a path buffer.
    ///
    /// # Returns
    ///
    /// A `PathBuf` containing the converted path.
    fn convert(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self)
    }
}

/// Implement the `OutputDirectory` trait for `&Path`.
impl OutputDirectory for &std::path::Path {
    /// Converts the reference to a path buffer.
    ///
    /// # Returns
    ///
    /// A `PathBuf` containing the converted path.
    fn convert(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self)
    }
}
