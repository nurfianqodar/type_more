/// Enum representing errors that can occur in the `TypeMore` crate.
///
/// This enum is used to classify different types of errors that might be encountered
/// during type operations, such as parsing errors or unexpected errors.
use std::fmt::Display;

#[derive(Debug)]
pub enum TypeMoreError {
    /// Represents an error that occurred due to an unexpected issue.
    UnhandledError(String),
    /// Represents an error that occurred due to a parsing issue.
    ParseError(String),
}

/// Implementation of the `Display` trait for `TypeMoreError`.
///
/// This trait allows `TypeMoreError` to be formatted as a string for user output or logging.
impl Display for TypeMoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "parse error! {}", msg),
            Self::UnhandledError(msg) => write!(f, "unhandled error! {}", msg),
        }
    }
}
