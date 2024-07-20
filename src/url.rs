// src/url.rs

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::TypeMoreError;

#[derive(Debug, Clone, PartialEq)]
pub enum Proto {
    Http,
    Https,
    Ftp,
    Sftp,
    Ftps,
    Ssh,
    Telnet,
    File,
    Ws,
    Wss,
    Gopher,
    Ldap,
    Rtsp,
    Smb,
    Nfs,
    Imap,
    Pop3,
    Nntp,
}

impl fmt::Display for Proto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proto::Http => write!(f, "http"),
            Proto::Https => write!(f, "https"),
            Proto::Ftp => write!(f, "ftp"),
            Proto::Sftp => write!(f, "sftp"),
            Proto::Ftps => write!(f, "ftps"),
            Proto::Ssh => write!(f, "ssh"),
            Proto::Telnet => write!(f, "telnet"),
            Proto::File => write!(f, "file"),
            Proto::Ws => write!(f, "ws"),
            Proto::Wss => write!(f, "wss"),
            Proto::Gopher => write!(f, "gopher"),
            Proto::Ldap => write!(f, "ldap"),
            Proto::Rtsp => write!(f, "rtsp"),
            Proto::Smb => write!(f, "smb"),
            Proto::Nfs => write!(f, "nfs"),
            Proto::Imap => write!(f, "imap"),
            Proto::Pop3 => write!(f, "pop3"),
            Proto::Nntp => write!(f, "nntp"),
        }
    }
}

// FromString for Proto
impl FromStr for Proto {
    type Err = TypeMoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "ftp" => Ok(Self::Ftp),
            "sftp" => Ok(Self::Sftp),
            "ftps" => Ok(Self::Ftps),
            "ssh" => Ok(Self::Ssh),
            "telnet" => Ok(Self::Telnet),
            "file" => Ok(Self::File),
            "ws" => Ok(Self::Ws),
            "wss" => Ok(Self::Wss),
            "gopher" => Ok(Self::Gopher),
            "ldap" => Ok(Self::Ldap),
            "rtsp" => Ok(Self::Rtsp),
            "smb" => Ok(Self::Smb),
            "nfs" => Ok(Self::Nfs),
            "imap" => Ok(Self::Imap),
            "pop3" => Ok(Self::Pop3),
            "nntp" => Ok(Self::Nntp),
            _ => Err(TypeMoreError::ParseError("invalid protocol".to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    protocol: Proto,
    domain: String,
}

// Constructor
impl Url {
    pub fn new(protocol: Proto, domain: impl ToString) -> Result<Self, TypeMoreError> {
        let domain_regex = Regex::new(
            r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}(?:/[a-zA-Z0-9._~!$&'()*+,;=:@%-]*)*$",
        ).unwrap();

        let domain = domain.to_string();

        if domain_regex.is_match(&domain) {
            Ok(Self { protocol, domain })
        } else {
            Err(TypeMoreError::ParseError("invalid domain".to_string()))
        }
    }
}

// Implement from sting
impl FromStr for Url {
    type Err = TypeMoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split("://").collect();
        if s.len() == 2 {
            let proto = Proto::from_str(s[0])?;
            let url = Self::new(proto, s[1])?;
            Ok(url)
        } else {
            Err(TypeMoreError::ParseError("invalid url".to_string()))
        }
    }
}

// Display
impl Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.protocol, self.domain)
    }
}

// Serde
impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Url::from_str(s).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
