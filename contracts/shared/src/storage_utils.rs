// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

//! Shared storage utilities for common database operations
//! 
//! This module provides reusable storage patterns and caching strategies
//! to reduce code duplication across contracts.

use soroban_sdk::{Env, Address, String, Vec, Symbol, symbol_short};

/// Default TTL values for caching
pub const DEFAULT_TEMP_TTL: u32 = 900; // 15 minutes
pub const DEFAULT_PERSISTENT_TTL_BUMP: u32 = 518400; // 6 days
pub const DEFAULT_PERSISTENT_TTL: u32 = 1036800; // 12 days

/// Common admin configuration retrieval pattern
pub fn get_admin_config_addresses(env: &Env, config_key: &Symbol) -> Option<Vec<Address>> {
    env.storage().persistent().get(config_key)
}

/// Common pattern for checking if user is in admin list
pub fn is_user_in_admin_list(
    env: &Env,
    user: &Address,
    admin_config_key: &Symbol,
) -> bool {
    if let Some(config) = get_admin_config_addresses(env, admin_config_key) {
        config.contains(user)
    } else {
        false
    }
}

/// Utility for generating sequential IDs
pub fn generate_sequential_id(env: &Env, counter_key: &Symbol) -> u128 {
    let current_id: u128 = env.storage().persistent().get(counter_key).unwrap_or(0);
    let new_id = current_id + 1;
    env.storage().persistent().set(counter_key, &new_id);
    new_id
}

/// Storage key helpers for common patterns
pub mod key_helpers {
    use super::*;
    
    /// Generate user-specific storage key
    pub fn user_key(prefix: &str, user: &Address) -> (Symbol, Address) {
        (Symbol::new(&Env::default(), prefix), user.clone())
    }
    
    /// Generate course-specific storage key
    pub fn course_key(prefix: &str, course_id: &String) -> (Symbol, String) {
        (Symbol::new(&Env::default(), prefix), course_id.clone())
    }
    
    /// Generate mapping key between two entities
    pub fn mapping_key_address_string(prefix: &str, first: &Address, second: &String) -> (Symbol, (Address, String)) {
        (Symbol::new(&Env::default(), prefix), (first.clone(), second.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_sequential_id_generation() {
        let env = Env::default();
        let counter_key = symbol_short!("counter");
        
        let id1 = generate_sequential_id(&env, &counter_key);
        let id2 = generate_sequential_id(&env, &counter_key);
        let id3 = generate_sequential_id(&env, &counter_key);
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_admin_list_check() {
        let env = Env::default();
        let config_key = symbol_short!("admins");
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Initially no admins
        assert!(!is_user_in_admin_list(&env, &user1, &config_key));
        
        // Add admin list
        let mut admins = Vec::new(&env);
        admins.push_back(user1.clone());
        env.storage().persistent().set(&config_key, &admins);
        
        // Check admin status
        assert!(is_user_in_admin_list(&env, &user1, &config_key));
        assert!(!is_user_in_admin_list(&env, &user2, &config_key));
    }
}
