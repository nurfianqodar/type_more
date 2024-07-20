use std::str::FromStr;

use type_more::url::{Proto, Url};

#[test]
fn test_proto_display() {
    assert_eq!(Proto::Http.to_string(), "http");
    assert_eq!(Proto::Https.to_string(), "https");
    assert_eq!(Proto::Ftp.to_string(), "ftp");
    assert_eq!(Proto::Sftp.to_string(), "sftp");
    assert_eq!(Proto::Ftps.to_string(), "ftps");
    assert_eq!(Proto::Ssh.to_string(), "ssh");
    assert_eq!(Proto::Telnet.to_string(), "telnet");
    assert_eq!(Proto::File.to_string(), "file");
    assert_eq!(Proto::Ws.to_string(), "ws");
    assert_eq!(Proto::Wss.to_string(), "wss");
    assert_eq!(Proto::Gopher.to_string(), "gopher");
    assert_eq!(Proto::Ldap.to_string(), "ldap");
    assert_eq!(Proto::Rtsp.to_string(), "rtsp");
    assert_eq!(Proto::Smb.to_string(), "smb");
    assert_eq!(Proto::Nfs.to_string(), "nfs");
    assert_eq!(Proto::Imap.to_string(), "imap");
    assert_eq!(Proto::Pop3.to_string(), "pop3");
    assert_eq!(Proto::Nntp.to_string(), "nntp");
}

#[test]
fn test_proto_from_str() {
    assert_eq!(Proto::from_str("http").unwrap(), Proto::Http);
    assert_eq!(Proto::from_str("https").unwrap(), Proto::Https);
    assert_eq!(Proto::from_str("ftp").unwrap(), Proto::Ftp);
    assert_eq!(Proto::from_str("sftp").unwrap(), Proto::Sftp);
    assert_eq!(Proto::from_str("ftps").unwrap(), Proto::Ftps);
    assert_eq!(Proto::from_str("ssh").unwrap(), Proto::Ssh);
    assert_eq!(Proto::from_str("telnet").unwrap(), Proto::Telnet);
    assert_eq!(Proto::from_str("file").unwrap(), Proto::File);
    assert_eq!(Proto::from_str("ws").unwrap(), Proto::Ws);
    assert_eq!(Proto::from_str("wss").unwrap(), Proto::Wss);
    assert_eq!(Proto::from_str("gopher").unwrap(), Proto::Gopher);
    assert_eq!(Proto::from_str("ldap").unwrap(), Proto::Ldap);
    assert_eq!(Proto::from_str("rtsp").unwrap(), Proto::Rtsp);
    assert_eq!(Proto::from_str("smb").unwrap(), Proto::Smb);
    assert_eq!(Proto::from_str("nfs").unwrap(), Proto::Nfs);
    assert_eq!(Proto::from_str("imap").unwrap(), Proto::Imap);
    assert_eq!(Proto::from_str("pop3").unwrap(), Proto::Pop3);
    assert_eq!(Proto::from_str("nntp").unwrap(), Proto::Nntp);
    assert!(Proto::from_str("invalid").is_err());
}

#[test]
fn test_url_new() {
    let url = Url::new(Proto::Http, "example.com").unwrap();
    assert_eq!(url.to_string(), "http://example.com");

    let url = Url::new(Proto::Https, "example.com/path").unwrap();
    assert_eq!(url.to_string(), "https://example.com/path");

    let invalid_url = Url::new(Proto::Http, "invalid_domain");
    assert!(invalid_url.is_err());
}

#[test]
fn test_url_from_str() {
    let url = Url::from_str("http://example.com").unwrap();
    assert_eq!(url.to_string(), "http://example.com");

    let url = Url::from_str("https://example.com/path").unwrap();
    assert_eq!(url.to_string(), "https://example.com/path");

    let invalid_url = Url::from_str("invalid_url");
    assert!(invalid_url.is_err());
}

#[test]
fn test_url_serde() {
    let url = Url::from_str("ftp://example.com").unwrap();
    let serialized = serde_json::to_string(&url).unwrap();
    assert_eq!(serialized, "\"ftp://example.com\"");

    let deserialized: Url = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.to_string(), "ftp://example.com");
    println!("{deserialized:?}")
}
