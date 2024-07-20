use std::fmt::Display;

#[derive(Debug)]
pub enum TypeMoreError {
    UnhandledError(String),
    ParseError(String),
}

impl Display for TypeMoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "parse error! {}", msg),
            Self::UnhandledError(msg) => write!(f, "unhandeled error! {}", msg),
        }
    }
}
