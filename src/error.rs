//! Typefrog errors.
use thiserror::Error;

#[derive(Error, Debug)]
/// Typefrog error variants.
pub enum TypefrogError {
    #[error("typeql error: {0}")]
    /// Parse error.
    TypeqlError(#[from] typeql::Error),
    #[error("database error: {0}")]
    /// Database error.
    DatabaseError(#[from] typedb_driver::Error),
    #[error("io error: {0}")]
    /// io error.
    IoError(#[from] std::io::Error),
    #[error("metadata error: {0}")]
    /// Metadata error.
    MetadataError(#[from] cargo_metadata::Error),
    #[error("serde error: {0}")]
    /// Serde error.
    SerdeError(#[from] serde_json::Error),
    #[error("syn error: {0}")]
    /// Syn error.
    SynError(#[from] syn::Error),
}
