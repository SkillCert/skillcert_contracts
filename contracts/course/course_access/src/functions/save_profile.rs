// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, String, Symbol, symbol_short};

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, UserProfile};

const SAVE_USER_PROFILE_EVENT: Symbol = symbol_short!("saveUsPrl");


/// Save a minimal on-chain user profile.
///
/// Stores only the user's address and an off-chain reference ID under
/// `DataKey::UserProfile(user)` in persistent storage and emits a
/// `saveUsPrl` event with `(user, off_chain_ref_id)`. All PII (name,
/// email, profession, goals, country) lives off-chain and is keyed by
/// `off_chain_ref_id`.
///
/// # Arguments
///
/// * `env` - The Soroban environment for storage and event access.
/// * `off_chain_ref_id` - UUID or hash mapping to the user's full record
///   in the off-chain database. Must be non-empty.
/// * `user` - The user's blockchain address that the profile belongs to.
///
/// # Returns
///
/// * `()` - The function returns the unit type on success. On failure
///   (empty `off_chain_ref_id`) it does not return — `handle_error`
///   raises `Error::OffChainRefIdRequired`, which halts contract
///   execution with the corresponding error code.
pub fn save_user_profile(env: Env, off_chain_ref_id: String, user: Address) {
    // Validate required field
    if off_chain_ref_id.is_empty() {
        handle_error(&env, Error::OffChainRefIdRequired)
    }

    let profile: UserProfile = UserProfile {
        user: user.clone(),
        off_chain_ref_id: off_chain_ref_id.clone(),
    };

    env.storage()
        .persistent()
        .set(&DataKey::UserProfile(user.clone()), &profile);

    env.events()
        .publish((SAVE_USER_PROFILE_EVENT,), (user, off_chain_ref_id));
}
