/// A module for handling the `Email` data type with validation and serialization/deserialization.
///
/// Uses a regular expression for email format validation and `serde` for
/// (de)serialization.
use crate::error::TypeMoreError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// A structure representing an email address.
///
/// # Structure
///
/// `Email` stores an email address as a string and provides various
/// implementations for validation, conversion, and serialization.
///
/// # Implementations
///
/// - `Constructor`: `Email::new(email: String) -> Result<Self, TypeMoreError>`
///   - Accepts an email string and returns an `Email` if the email is valid.
/// - `FromStr`: Implements the `FromStr` trait for conversion from string to `Email`.
/// - `Display`: Implements the `Display` trait for printing `Email` as a string.
/// - `Serialize`: Implements the `Serialize` trait for serialization to a string format.
/// - `Deserialize`: Implements the `Deserialize` trait for deserialization from a string format.
#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

/// Implementation of the constructor and validation for `Email`.
impl Email {
    /// Creates a new instance of `Email`.
    ///
    /// # Parameters
    ///
    /// - `email`: A string representing the email address.
    ///
    /// # Returns
    ///
    /// - `Ok(Self(email))`: If the email is valid.
    /// - `Err(TypeMoreError::ParseError("invalid email".to_string()))`: If the email is invalid.
    ///
    /// # Errors
    ///
    /// - `TypeMoreError::UnhandledError("invalid regex pattern".to_string())`: If the regex pattern is invalid.
    pub fn new(email: String) -> Result<Self, TypeMoreError> {
        // Define regex pattern outside the function if optimization is needed
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| TypeMoreError::UnhandledError("invalid regex pattern".to_string()))?;
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(TypeMoreError::ParseError("invalid email".to_string()))
        }
    }
}

/// Implementation of the `FromStr` trait for converting from string to `Email`.
impl FromStr for Email {
    type Err = TypeMoreError;

    /// Converts a string to an `Email`.
    ///
    /// # Parameters
    ///
    /// - `s`: A string representing the email address.
    ///
    /// # Returns
    ///
    /// - `Ok(Self(email))`: If the conversion is successful.
    /// - `Err(TypeMoreError::ParseError("invalid email".to_string()))`: If the string is not a valid email.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Implementation of the `Display` trait for printing `Email` as a string.
impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implementation of the `Serialize` trait for serializing `Email`.
impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Implementation of the `Deserialize` trait for deserializing `Email`.
impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Email::from_str(s).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}
