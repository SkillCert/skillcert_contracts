// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

//! Shared contract versioning functionality
//! 
//! This module provides common versioning and migration functionality
//! that can be used across all SkillCert contracts.

use soroban_sdk::{contracterror, Address, Env, String, Vec, vec};

/// Errors that can occur during contract versioning operations
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VersioningError {
    /// Invalid version format
    InvalidVersion = 1,
    /// Version not found in history
    VersionNotFound = 2,
    /// Migration not compatible
    MigrationNotCompatible = 3,
    /// Migration already completed
    MigrationAlreadyCompleted = 4,
    /// Unauthorized migration attempt
    UnauthorizedMigration = 5,
    /// Migration failed
    MigrationFailed = 6,
}

/// Storage keys for versioning data
const VERSION_HISTORY_KEY: &str = "version_history";
const MIGRATION_STATUS_KEY: &str = "migration_status";

/// Trait for contract-specific migration logic
pub trait MigrationHandler {
    /// Check if the caller is authorized to perform migration
    fn is_authorized_for_migration(env: &Env, caller: &Address) -> bool;
    
    /// Perform contract-specific data migration
    fn perform_data_migration(env: &Env, from_version: &String, to_version: &String) -> bool;
    
    /// Get the contract-specific event prefix for migration events
    fn get_migration_event_prefix() -> &'static str;
}

/// Get the version history of the contract
pub fn get_version_history(env: &Env) -> Vec<String> {
    let key = String::from_str(env, VERSION_HISTORY_KEY);
    env.storage()
        .instance()
        .get::<String, Vec<String>>(&key)
        .unwrap_or_else(|| vec![env])
}

/// Store a new version in the history
pub fn store_version_in_history(env: &Env, version: String) {
    let mut history = get_version_history(env);
    history.push_back(version.clone());
    
    let key = String::from_str(env, VERSION_HISTORY_KEY);
    env.storage().instance().set(&key, &history);
}

/// Check if a version exists in the history
pub fn version_exists_in_history(env: &Env, version: &String) -> bool {
    let history = get_version_history(env);
    for v in history.iter() {
        if &v == version {
            return true;
        }
    }
    false
}

/// Get migration status information
pub fn get_migration_status(env: &Env) -> String {
    let key = String::from_str(env, MIGRATION_STATUS_KEY);
    env.storage()
        .instance()
        .get::<String, String>(&key)
        .unwrap_or_else(|| String::from_str(env, "No migrations pending"))
}

/// Set migration status
pub fn set_migration_status(env: &Env, status: String) {
    let key = String::from_str(env, MIGRATION_STATUS_KEY);
    env.storage().instance().set(&key, &status);
}

/// Check compatibility between two versions
pub fn is_version_compatible(_env: &Env, _from_version: String, _to_version: String) -> bool {
    // Simple compatibility check - for now, assume all versions are compatible
    // In a real implementation, you would parse semantic versions properly
    true
}

/// Generic migration function that uses the MigrationHandler trait
pub fn migrate_contract_data<T: MigrationHandler>(
    env: &Env,
    caller: Address,
    from_version: String,
    to_version: String,
) -> bool {
    // Check if caller is authorized using contract-specific logic
    if !T::is_authorized_for_migration(env, &caller) {
        set_migration_status(env, String::from_str(env, "Migration failed: Unauthorized"));
        return false;
    }
    
    // Validate versions exist in history
    if !version_exists_in_history(env, &from_version) {
        set_migration_status(env, String::from_str(env, "Migration failed: Source version not found"));
        return false;
    }
    
    // Check compatibility
    if !is_version_compatible(env, from_version.clone(), to_version.clone()) {
        set_migration_status(env, String::from_str(env, "Migration failed: Versions not compatible"));
        return false;
    }
    
    // Perform migration using contract-specific logic
    let migration_result = T::perform_data_migration(env, &from_version, &to_version);
    
    if migration_result {
        // Update version history with new version
        store_version_in_history(env, to_version.clone());
        
        // Set successful migration status
        let status = String::from_str(env, "Migration completed successfully");
        set_migration_status(env, status);
        
        // Emit migration event
        emit_migration_event::<T>(env, &from_version, &to_version, true);
        
        true
    } else {
        set_migration_status(env, String::from_str(env, "Migration failed: Data transformation error"));
        emit_migration_event::<T>(env, &from_version, &to_version, false);
        false
    }
}

/// Emit a migration event with contract-specific prefix
pub fn emit_migration_event<T: MigrationHandler>(
    _env: &Env, 
    _from_version: &String, 
    _to_version: &String, 
    _success: bool
) {
    // In a real implementation, you would emit events here
    // For now, we'll just set a status message
    
    let _event_type = if _success { "success" } else { "failure" };
    let _event_prefix = T::get_migration_event_prefix();
    
    // In a real implementation, you would emit actual events here
    // For now, we'll just store a simple status message
    
    // You could emit actual events here using env.events()
    // env.events().publish((event_prefix, event_type), (from_version, to_version, success));
}

/// Utility function for version 1.0.0 to 1.1.0 migration pattern
/// This can be used as a template for common migration scenarios
pub fn migrate_v1_0_0_to_v1_1_0(_env: &Env) -> bool {
    // Placeholder for common migration logic
    // This would typically involve:
    // 1. Reading existing data structures
    // 2. Adding new fields with default values
    // 3. Saving updated data structures
    
    // For now, return true to indicate successful migration
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    // Mock implementation for testing
    struct TestMigrationHandler;

    impl MigrationHandler for TestMigrationHandler {
        fn is_authorized_for_migration(_env: &Env, _caller: &Address) -> bool {
            true // Allow all for testing
        }
        
        fn perform_data_migration(_env: &Env, _from_version: &String, _to_version: &String) -> bool {
            true // Simulate successful migration
        }
        
        fn get_migration_event_prefix() -> &'static str {
            "test_migration"
        }
    }

    #[test]
    fn test_version_history() {
        let env = Env::default();
        
        // Initially empty
        let history = get_version_history(&env);
        assert_eq!(history.len(), 0);
        
        // Add a version
        let version = String::from_str(&env, "1.0.0");
        store_version_in_history(&env, version.clone());
        
        let updated_history = get_version_history(&env);
        assert_eq!(updated_history.len(), 1);
        assert!(version_exists_in_history(&env, &version));
    }

    #[test]
    fn test_version_compatibility() {
        let env = Env::default();
        
        // All versions are compatible in our simplified implementation
        assert!(is_version_compatible(&env, 
            String::from_str(&env, "1.0.0"), 
            String::from_str(&env, "1.1.0")));
        
        assert!(is_version_compatible(&env, 
            String::from_str(&env, "1.0.0"), 
            String::from_str(&env, "2.0.0")));
    }

    #[test]
    fn test_migration_status() {
        let env = Env::default();
        
        // Initially no migrations pending
        let initial_status = get_migration_status(&env);
        assert_eq!(initial_status, String::from_str(&env, "No migrations pending"));
        
        // Set a status
        let new_status = String::from_str(&env, "Migration in progress");
        set_migration_status(&env, new_status.clone());
        
        let updated_status = get_migration_status(&env);
        assert_eq!(updated_status, new_status);
    }

    #[test]
    fn test_migrate_contract_data() {
        let env = Env::default();
        let caller = Address::generate(&env);
        
        // First add the source version to history
        let from_version = String::from_str(&env, "1.0.0");
        store_version_in_history(&env, from_version.clone());
        
        let to_version = String::from_str(&env, "1.1.0");
        
        // Test successful migration
        let result = migrate_contract_data::<TestMigrationHandler>(
            &env, 
            caller, 
            from_version, 
            to_version.clone()
        );
        
        assert!(result);
        assert!(version_exists_in_history(&env, &to_version));
        
        let status = get_migration_status(&env);
        assert_eq!(status, String::from_str(&env, "Migration completed successfully"));
    }
}
