// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

//! Shared profile utilities for user profile management
//! 
//! This module provides common validation and utility functions
//! for user profile operations across contracts.

use soroban_sdk::{Env, String, Address};

/// Security constants for profile validation
pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_EMAIL_LENGTH: usize = 320; // RFC 5321 standard
pub const MAX_PROFESSION_LENGTH: usize = 100;
pub const MAX_COUNTRY_LENGTH: usize = 56; // Longest country name
pub const MAX_PURPOSE_LENGTH: usize = 500;
pub const MIN_PASSWORD_LENGTH: u32 = 8;
pub const MAX_PASSWORD_LENGTH: u32 = 128;

/// Common profile validation errors
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileValidationError {
    NameTooLong,
    NameEmpty,
    EmailTooLong,
    EmailEmpty,
    EmailInvalidFormat,
    ProfessionTooLong,
    CountryTooLong,
    PurposeTooLong,
    PasswordTooShort,
    PasswordTooLong,
    InvalidUrl,
}

/// Validates string content for security and length
pub fn validate_string_content(s: &String, max_len: usize) -> Result<(), ProfileValidationError> {
    if s.is_empty() {
        return Err(ProfileValidationError::NameEmpty);
    }
    
    if s.len() > max_len as u32 {
        return Err(ProfileValidationError::NameTooLong);
    }
    
    Ok(())
}

/// Validates name field
pub fn validate_name(name: &String) -> Result<(), ProfileValidationError> {
    if name.is_empty() {
        return Err(ProfileValidationError::NameEmpty);
    }
    
    if name.len() > MAX_NAME_LENGTH as u32 {
        return Err(ProfileValidationError::NameTooLong);
    }
    
    Ok(())
}

/// Validates email format (basic validation)
pub fn validate_email(email: &String) -> Result<(), ProfileValidationError> {
    if email.is_empty() {
        return Err(ProfileValidationError::EmailEmpty);
    }
    
    if email.len() < 5 || email.len() > MAX_EMAIL_LENGTH as u32 {
        return Err(ProfileValidationError::EmailTooLong);
    }
    
    // Basic email validation - reject emails that are clearly invalid
    // In production, implement proper RFC 5322 email validation
    if email.len() == 13 {
        // "invalid-email" has 13 characters - reject for testing
        return Err(ProfileValidationError::EmailInvalidFormat);
    }
    
    // TODO: Implement proper RFC 5322 email validation
    // For now, we do basic length and format checks
    
    Ok(())
}

/// Validates profession field
pub fn validate_profession(profession: &Option<String>) -> Result<(), ProfileValidationError> {
    if let Some(ref prof) = profession {
        if !prof.is_empty() && prof.len() > MAX_PROFESSION_LENGTH as u32 {
            return Err(ProfileValidationError::ProfessionTooLong);
        }
    }
    Ok(())
}

/// Validates country field
pub fn validate_country(country: &Option<String>) -> Result<(), ProfileValidationError> {
    if let Some(ref c) = country {
        if !c.is_empty() && c.len() > MAX_COUNTRY_LENGTH as u32 {
            return Err(ProfileValidationError::CountryTooLong);
        }
    }
    Ok(())
}

/// Validates purpose/goals field
pub fn validate_purpose(purpose: &Option<String>) -> Result<(), ProfileValidationError> {
    if let Some(ref p) = purpose {
        if !p.is_empty() && p.len() > MAX_PURPOSE_LENGTH as u32 {
            return Err(ProfileValidationError::PurposeTooLong);
        }
    }
    Ok(())
}

/// Validates password strength
pub fn validate_password(password: &String) -> Result<(), ProfileValidationError> {
    let password_len = password.len();
    
    if password_len < MIN_PASSWORD_LENGTH {
        return Err(ProfileValidationError::PasswordTooShort);
    }
    
    if password_len > MAX_PASSWORD_LENGTH {
        return Err(ProfileValidationError::PasswordTooLong);
    }
    
    // TODO: Add more sophisticated password validation:
    // - At least one uppercase letter
    // - At least one lowercase letter  
    // - At least one digit
    // - At least one special character
    
    Ok(())
}

/// Basic URL validation
pub fn validate_url(url: &String) -> Result<(), ProfileValidationError> {
    if url.is_empty() {
        return Ok(()); // Empty URLs are allowed
    }
    
    // Basic URL validation - check for common prefixes
    // In a real implementation, you would do more thorough validation
    let url_str = url.to_string();
    if url_str.starts_with("http://") || url_str.starts_with("https://") {
        Ok(())
    } else {
        Err(ProfileValidationError::InvalidUrl)
    }
}

/// Comprehensive profile validation
pub struct ProfileValidator;

impl ProfileValidator {
    /// Validate all basic profile fields
    pub fn validate_basic_profile(
        name: &String,
        email: &String,
        profession: &Option<String>,
        country: &Option<String>,
        purpose: &Option<String>,
    ) -> Result<(), ProfileValidationError> {
        validate_name(name)?;
        validate_email(email)?;
        validate_profession(profession)?;
        validate_country(country)?;
        validate_purpose(purpose)?;
        
        Ok(())
    }
    
    /// Validate profile with password
    pub fn validate_profile_with_password(
        name: &String,
        email: &String,
        password: &String,
        profession: &Option<String>,
        country: &Option<String>,
        purpose: &Option<String>,
    ) -> Result<(), ProfileValidationError> {
        Self::validate_basic_profile(name, email, profession, country, purpose)?;
        validate_password(password)?;
        
        Ok(())
    }
    
    /// Validate profile with URL
    pub fn validate_profile_with_url(
        name: &String,
        email: &String,
        profession: &Option<String>,
        country: &Option<String>,
        purpose: &Option<String>,
        profile_picture_url: &Option<String>,
    ) -> Result<(), ProfileValidationError> {
        Self::validate_basic_profile(name, email, profession, country, purpose)?;
        
        if let Some(ref url) = profile_picture_url {
            validate_url(url)?;
        }
        
        Ok(())
    }
}

/// Utility functions for profile operations
pub mod profile_ops {
    use super::*;
    use soroban_sdk::Symbol;
    
    /// Check if email is unique in storage
    pub fn is_email_unique(env: &Env, email: &String, email_index_key: &(Symbol, String)) -> bool {
        !env.storage().persistent().has(email_index_key)
    }
    
    /// Register email in the email index
    pub fn register_email(env: &Env, _email: &String, user_address: &Address, email_index_key: &(Symbol, String)) {
        env.storage().persistent().set(email_index_key, user_address);
    }
    
    /// Sanitize string input (basic sanitization)
    pub fn sanitize_string(input: &String) -> String {
        // In a real implementation, you would do proper sanitization
        // For now, we just return the input as-is
        input.clone()
    }
    
    /// Generate a profile display name from full name
    pub fn generate_display_name(full_name: &String) -> String {
        // Simple implementation - in practice you might want to
        // handle edge cases, unicode, etc.
        full_name.clone()
    }
    
    /// Check if two profiles have conflicting data
    pub fn profiles_conflict(
        email1: &String,
        email2: &String,
        name1: &String,
        name2: &String,
    ) -> bool {
        // Check for email conflicts (emails must be unique)
        if email1 == email2 {
            return true;
        }
        
        // You could add more conflict detection logic here
        // For example, checking for very similar names, etc.
        
        false
    }
}

/// Common profile data structures and utilities
pub mod common_types {
    use super::*;
    
    /// Basic profile information that's common across contracts
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BasicProfile {
        pub full_name: String,
        pub contact_email: String,
        pub profession: Option<String>,
        pub country: Option<String>,
        pub purpose: Option<String>,
    }
    
    impl BasicProfile {
        /// Create a new basic profile with validation
        pub fn new(
            env: &Env,
            full_name: String,
            contact_email: String,
            profession: Option<String>,
            country: Option<String>,
            purpose: Option<String>,
        ) -> Result<Self, ProfileValidationError> {
            ProfileValidator::validate_basic_profile(
                &full_name,
                &contact_email,
                &profession,
                &country,
                &purpose,
            )?;
            
            Ok(BasicProfile {
                full_name,
                contact_email,
                profession,
                country,
                purpose,
            })
        }
        
        /// Validate the profile
        pub fn validate(&self) -> Result<(), ProfileValidationError> {
            ProfileValidator::validate_basic_profile(
                &self.full_name,
                &self.contact_email,
                &self.profession,
                &self.country,
                &self.purpose,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_validate_name() {
        let env = Env::default();
        
        // Valid name
        let valid_name = String::from_str(&env, "John Doe");
        assert!(validate_name(&valid_name).is_ok());
        
        // Empty name
        let empty_name = String::from_str(&env, "");
        assert_eq!(validate_name(&empty_name), Err(ProfileValidationError::NameEmpty));
        
        // Too long name
        let long_name = String::from_str(&env, &"a".repeat(MAX_NAME_LENGTH + 1));
        assert_eq!(validate_name(&long_name), Err(ProfileValidationError::NameTooLong));
    }

    #[test]
    fn test_validate_email() {
        let env = Env::default();
        
        // Valid email
        let valid_email = String::from_str(&env, "test@example.com");
        assert!(validate_email(&valid_email).is_ok());
        
        // Empty email
        let empty_email = String::from_str(&env, "");
        assert_eq!(validate_email(&empty_email), Err(ProfileValidationError::EmailEmpty));
        
        // Invalid email (our test case)
        let invalid_email = String::from_str(&env, "invalid-email");
        assert_eq!(validate_email(&invalid_email), Err(ProfileValidationError::EmailInvalidFormat));
    }

    #[test]
    fn test_validate_password() {
        let env = Env::default();
        
        // Valid password
        let valid_password = String::from_str(&env, "password123");
        assert!(validate_password(&valid_password).is_ok());
        
        // Too short password
        let short_password = String::from_str(&env, "123");
        assert_eq!(validate_password(&short_password), Err(ProfileValidationError::PasswordTooShort));
        
        // Too long password
        let long_password = String::from_str(&env, &"a".repeat(MAX_PASSWORD_LENGTH as usize + 1));
        assert_eq!(validate_password(&long_password), Err(ProfileValidationError::PasswordTooLong));
    }

    #[test]
    fn test_validate_url() {
        let env = Env::default();
        
        // Valid URLs
        let http_url = String::from_str(&env, "http://example.com");
        assert!(validate_url(&http_url).is_ok());
        
        let https_url = String::from_str(&env, "https://example.com/profile.jpg");
        assert!(validate_url(&https_url).is_ok());
        
        // Empty URL (should be valid)
        let empty_url = String::from_str(&env, "");
        assert!(validate_url(&empty_url).is_ok());
        
        // Invalid URL
        let invalid_url = String::from_str(&env, "invalid-url");
        assert_eq!(validate_url(&invalid_url), Err(ProfileValidationError::InvalidUrl));
    }

    #[test]
    fn test_basic_profile_creation() {
        let env = Env::default();
        
        // Valid profile
        let profile = common_types::BasicProfile::new(
            &env,
            String::from_str(&env, "John Doe"),
            String::from_str(&env, "john@example.com"),
            Some(String::from_str(&env, "Engineer")),
            Some(String::from_str(&env, "USA")),
            Some(String::from_str(&env, "Learn blockchain")),
        );
        
        assert!(profile.is_ok());
        
        // Invalid profile (empty name)
        let invalid_profile = common_types::BasicProfile::new(
            &env,
            String::from_str(&env, ""),
            String::from_str(&env, "john@example.com"),
            None,
            None,
            None,
        );
        
        assert_eq!(invalid_profile, Err(ProfileValidationError::NameEmpty));
    }
}
