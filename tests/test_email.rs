// tests/test_email.rs

use type_more::Email;

#[test]
fn test_valid_email_creation() {
    let email = Email::new("test@example.com".to_string());
    assert!(email.is_ok());
    let email = email.unwrap();
    assert_eq!(email.to_string(), "test@example.com");
}

#[test]
fn test_invalid_email_creation() {
    let email = Email::new("invalid-email".to_string());
    assert!(email.is_err());
}

#[test]
fn test_email_serialization() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let serialized = serde_json::to_string(&email).unwrap();
    assert_eq!(serialized, "\"test@example.com\"");
}

#[test]
fn test_email_deserialization() {
    let json_str = "\"test@example.com\"";
    let deserialized: Email = serde_json::from_str(json_str).unwrap();
    assert_eq!(deserialized.to_string(), "test@example.com");
}

#[test]
fn test_invalid_email_deserialization() {
    let json_str = "\"invalid-email\"";
    let result: Result<Email, _> = serde_json::from_str(json_str);
    assert!(result.is_err());
}

#[test]
fn test_display_implementation() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    assert_eq!(format!("{}", email), "test@example.com");
}
