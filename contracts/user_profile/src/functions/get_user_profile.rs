// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert
use soroban_sdk::{Address, Env, Symbol};
use crate::schema::UserProfile;

 validate-input-params
 validate-input-params
/// Get user profile by address (public function)
pub fn get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // Uncomment and use handle_error if Address can be empty:
    // if user_address.is_empty() {
    //     handle_error(env, Error::InvalidInput);
    // }
    let key = Symbol::new(env, "profile");

pub fn get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");

    // Get the user profile from storage
 main
=======
pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // For demonstration, assume Address cannot be empty.
    
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");
    let storage_key = (key, user_address.clone());
    
    // Try temporary storage first for frequently accessed profiles
    if let Some(profile) = env.storage().temporary().get(&storage_key) {
        return profile;
    }
    // Get from instance storage if not cached
 main
    let profile: UserProfile = env
        .storage()
        .instance()
        .get(&storage_key)
        .expect("User profile not found");
 validate-input-params
validate-input-params

 main

    // Cache in temporary storage for subsequent requests
    env.storage().temporary().set(&storage_key, &profile);
    // Cache for 15 minutes
    env.storage().temporary().extend_ttl(&storage_key, 0, 900);
  main
    profile
}

// Function to get user profile with privacy check
// Returns profile only if it's public or if the requester is the profile owner validate-input-params
/// Get user profile with privacy check
/// Returns profile only if it's public or if the requester is the profile owner
 main
pub fn get_user_profile_with_privacy(
    env: &Env,
    user_address: Address,
    requester_address: Address,
) -> UserProfile {
 validate-input-params
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // Uncomment and use handle_error if Address can be empty:
    // if user_address.is_empty() {
    //     handle_error(env, Error::InvalidInput);
    // }
    let key = Symbol::new(env, "profile");
 validate-input-params


    // TODO: Implement caching mechanism for frequently accessed profiles

    // Get the user profile from storage
 main
    let mut profile: UserProfile = env
        .storage()
        .instance()
        .get(&(key, user_address.clone()))
        .expect("User profile not found");
 validate-input-params
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
    }


    // Check privacy settings
    // If profile is not public and requester is not the profile owner, hide email

    // Reuse the optimized get_user_profile function
    let mut profile = user_profile_get_user_profile(env, user_address.clone());
    // Apply privacy filters without additional storage reads
 main
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
        // Add more privacy filters as needed
    }
 validate-input-params
 main

 main
    profile
