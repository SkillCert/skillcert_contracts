// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Env, Address, Symbol};

use crate::schema::UserProfile;

pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // For demonstration, assume Address cannot be empty.
    // Consistent error handling for invalid input
    // Uncomment and use handle_error if Address can be empty:
    // if user_address.is_empty() {
    //     handle_error(env, Error::InvalidInput);
    // }
    let key = Symbol::new(env, "profile");
    let profile: UserProfile = env
        .storage()
        .instance()
        .get(&(key, user_address.clone()))
        .expect("User profile not found");
    profile
}

// Function to get user profile with privacy check
// Returns profile only if it's public or if the requester is the profile owner
pub fn user_profile_get_user_profile_with_privacy(
    env: &Env, 
    user_address: Address, 
    requester_address: Address
) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // Consistent error handling for invalid input
    // Uncomment and use handle_error if Address can be empty:
    // if user_address.is_empty() {
    //     handle_error(env, Error::InvalidInput);
    // }
    let key = Symbol::new(env, "profile");
    let mut profile: UserProfile = env
        .storage()
        .instance()
        .get(&(key, user_address.clone()))
        .expect("User profile not found");
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
    }
    profile
}
