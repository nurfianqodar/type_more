# TYPE_MORE PROJECT DEVELOPMENT GUIDE

## TODO Types!

### Email

- [X] Create regex pattern for email
  - [X] Define a regex pattern that matches standard email formats.
  - [X] Ensure the pattern covers common edge cases.
- [X] Parse from `impl ToString`
  - [X] Implement `FromStr` trait for email parsing.
  - [X] Handle invalid email formats gracefully.
- [X] Serialize and Deserialize as String
  - [X] Implement `serde::Serialize` and `serde::Deserialize`.
  - [X] Test serialization and deserialization.
- [X] Implement Display to String
  - [X] Implement `fmt::Display` for email.
  - [X] Ensure the email displays correctly in all scenarios.
- [ ] Create documentation

### Mac Address

- [ ] Create regex pattern for MAC address
  - [ ] Define a regex pattern that matches MAC address formats.
  - [ ] Ensure the pattern covers different MAC address formats (e.g., `00:1A:2B:3C:4D:5E`, `001A.2B3C.4D5E`).
- [ ] Parse from `impl ToString`
  - [ ] Implement `FromStr` trait for MAC address parsing.
  - [ ] Handle invalid MAC address formats gracefully.
- [ ] Serialize and Deserialize as String
  - [ ] Implement `serde::Serialize` and `serde::Deserialize`.
  - [ ] Test serialization and deserialization.
- [ ] Implement Display to String
  - [ ] Implement `fmt::Display` for MAC address.
  - [ ] Ensure the MAC address displays correctly in all scenarios.

### Money

- [ ] Define Money struct
  - [ ] Include fields for amount and currency.
- [ ] Implement arithmetic operations
  - [ ] Add support for addition, subtraction, multiplication, and division.
  - [ ] Ensure operations handle different currencies correctly.
- [ ] Implement currency conversion
  - [ ] Add support for currency conversion.
  - [ ] Use a placeholder function for exchange rates (to be implemented later).
- [ ] Serialize and Deserialize
  - [ ] Implement `serde::Serialize` and `serde::Deserialize`.
  - [ ] Test serialization and deserialization.
- [ ] Implement Display to String
  - [ ] Implement `fmt::Display` for Money.
  - [ ] Ensure the Money displays correctly in all scenarios.

### Password

- [ ] Define RawPassword and HashedPassword structs
  - [ ] Implement fields for storing raw and hashed passwords.
- [ ] Implement hashing for RawPassword
  - [ ] Use a secure hashing algorithm (e.g., bcrypt, argon2).
  - [ ] Ensure hashing includes salt for security.
- [ ] Implement verification for HashedPassword
  - [ ] Add a method to verify raw passwords against hashed passwords.
  - [ ] Ensure verification is secure and efficient.
- [ ] Serialize and Deserialize
  - [ ] Implement `serde::Serialize` and `serde::Deserialize`.
  - [ ] Test serialization and deserialization.
- [ ] Implement Display to String
  - [ ] Implement `fmt::Display` for RawPassword and HashedPassword.
  - [ ] Ensure the passwords display correctly in all scenarios.

### Phone

- [ ] Create regex pattern for phone numbers
  - [ ] Define a regex pattern that matches international phone number formats.
  - [ ] Ensure the pattern covers different phone number formats (e.g., `+1-800-555-5555`, `0800-555-555`).
- [ ] Parse from `impl ToString`
  - [ ] Implement `FromStr` trait for phone number parsing.
  - [ ] Handle invalid phone number formats gracefully.
- [ ] Serialize and Deserialize as String
  - [ ] Implement `serde::Serialize` and `serde::Deserialize`.
  - [ ] Test serialization and deserialization.
- [ ] Implement Display to String
  - [ ] Implement `fmt::Display` for phone numbers.
  - [ ] Ensure the phone number displays correctly in all scenarios.

### Url

- [X] Parse from `impl ToString`
  - [X] Implement `FromStr` trait for URL parsing.
  - [X] Handle invalid URL formats gracefully.
- [X] Serialize and Deserialize as String
  - [X] Implement `serde::Serialize` and `serde::Deserialize`.
  - [X] Test serialization and deserialization.
- [X] Implement Display to String
  - [X] Implement `fmt::Display` for URLs.
  - [X] Ensure the URL displays correctly in all scenarios.
- [ ] Create Documentation

## Additional Tasks

### Testing

- [ ] Write comprehensive unit tests for each type.
  - [ ] Cover all edge cases and ensure full code coverage.
- [ ] Write integration tests for crate functionality.
  - [ ] Ensure different types interact correctly and as expected.

### Documentation

- [ ] Write comprehensive documentation for each type.
  - [ ] Include examples and usage guidelines.
- [ ] Create README.md with an overview of the crate and how to use it.
- [ ] Add comments to the code to explain complex logic and decisions.

### CI/CD Setup

- [ ] Set up Continuous Integration (CI) for automated testing.
  - [ ] Use services like GitHub Actions or Travis CI.
- [ ] Set up Continuous Deployment (CD) for automated publishing.
  - [ ] Ensure the crate is published to crates.io on new releases.
