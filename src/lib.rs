/// # `type_more`
///
/// `type_more` is a Rust crate that provides custom data types such as `Email`, `Url`, and others. It follows the "parse, don't validate" principle to ensure that data is parsed correctly while avoiding unnecessary validation logic. This approach focuses on parsing data into the desired format rather than validating its correctness.
///
/// ## Features
///
/// - **Email**: A custom type for handling email addresses.
/// - **Url**: A custom type for handling URLs.
///
/// ## Design Principles
///
/// ### Parse, Don't Validate
///
/// The crate adheres to the "parse, don't validate" principle. This means that instead of validating data against specific rules or patterns, the crate focuses on parsing the data into structured types. This approach ensures that the data is correctly formatted and can be used in a consistent manner within the application.
///
/// ### Example
///
/// **Note:** This section is intentionally left out as per the request. For usage examples, refer to the [documentation](https://docs.rs/type_more).
///
/// ## Installation
///
/// Add `type_more` to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// type_more = "0.1" # Replace with the latest version
///
mod email;
pub mod error;
mod password;
mod url;

pub use email::*;
pub use password::*;
pub use url::*;
