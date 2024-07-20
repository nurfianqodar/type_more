// src/email.rs

use crate::error::TypeMoreError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

// Constructor
impl Email {
    pub fn new(email: String) -> Result<Self, TypeMoreError> {
        // Tentukan regex pattern di luar fungsi jika perlu dioptimalkan
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| TypeMoreError::UnhandledError("invalid regex pattern".to_string()))?;
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(TypeMoreError::ParseError("invalid email".to_string()))
        }
    }
}

// From String
impl FromStr for Email {
    type Err = TypeMoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

// Display
impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Serde
impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Email::from_str(s).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}
