use std::marker::PhantomData;

use bcrypt::BcryptError;

use crate::error::TypeMoreError;

pub enum PasswordStrength {
    Weak,
    Moderate,
    Strong,
    Extreme,
}

pub struct RawPassword {
    strength: PasswordStrength,
    password: String,
}

// methods
impl RawPassword {
    pub fn new_weak(password: impl ToString) -> Result<Self, TypeMoreError> {
        let password = password.to_string();
        if password.len() < 8 {
            Err(TypeMoreError::ParseError(
                "invalid weak password".to_string(),
            ))
        } else {
            Ok(Self {
                password,
                strength: PasswordStrength::Weak,
            })
        }
    }

    pub fn new_moderate(password: impl ToString) -> Result<Self, TypeMoreError> {
        let password = password.to_string();
        if password.len() < 8 && !(password.chars().any(|c| c.is_ascii_digit())) {
            Err(TypeMoreError::ParseError(
                "invalid moderate password".to_string(),
            ))
        } else {
            Ok(Self {
                password,
                strength: PasswordStrength::Weak,
            })
        }
    }

    pub fn new_strong(password: impl ToString) -> Result<Self, TypeMoreError> {
        let password = password.to_string();
        if password.len() < 8
            && !(password.chars().any(|c| c.is_ascii_digit()))
            && !(password.chars().any(|c| c.is_uppercase()))
        {
            Err(TypeMoreError::ParseError(
                "invalid strong password".to_string(),
            ))
        } else {
            Ok(Self {
                password,
                strength: PasswordStrength::Weak,
            })
        }
    }

    pub fn new_extreme(password: impl ToString) -> Result<Self, TypeMoreError> {
        let password = password.to_string();
        if password.len() < 10
            && !(password.chars().any(|c| c.is_ascii_digit()))
            && !(password.chars().any(|c| c.is_uppercase()))
            && !(password.chars().any(|c| "~`@#$%^&*()_+".contains(c)))
        {
            Err(TypeMoreError::ParseError(
                "invalid extreme password".to_string(),
            ))
        } else {
            Ok(Self {
                password,
                strength: PasswordStrength::Weak,
            })
        }
    }

    pub fn hash_bcrypt(self) -> Result<HashedPassword<Bcrypt>, BcryptError> {
        HashedPassword::new(self.password, None)
    }

    pub fn get_strength(&self) -> String {
        match &self.strength {
            PasswordStrength::Weak => "weak".to_string(),
            PasswordStrength::Moderate => "moderate".to_string(),
            PasswordStrength::Strong => "strong".to_string(),
            PasswordStrength::Extreme => "extreme".to_string(),
        }
    }
}

// Hashed password
pub trait HashAlgorithm {}
pub struct Bcrypt;
pub struct Argon2;
impl HashAlgorithm for Bcrypt {}
impl HashAlgorithm for Argon2 {}

pub struct HashedPassword<T: HashAlgorithm> {
    algorithm: PhantomData<T>,
    password: String,
}

impl HashedPassword<Bcrypt> {
    pub fn new(password: impl ToString, cost: Option<u32>) -> Result<Self, BcryptError> {
        let raw_pass = password.to_string();
        let cost = match cost {
            Some(c) => c,
            None => bcrypt::DEFAULT_COST,
        };

        let hashed = bcrypt::hash(raw_pass, cost)?;
        Ok(Self {
            algorithm: PhantomData,
            password: hashed.to_string(),
        })
    }

    pub fn verify(&self, password: impl ToString) -> Result<bool, BcryptError> {
        let raw_password = password.to_string();
        let is_verified = bcrypt::verify(raw_password, &self.password)?;
        Ok(is_verified)
    }
}
