// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::Error;
use soroban_sdk::{Address, Env, Symbol};

const KEY_USER_MGMT_ADDR: &str = "user_mgmt_addr";

/// Check if a user is an admin through cross-contract call
pub fn is_admin(env: &Env, who: &Address) -> bool {
    let user_mgmt_addr: Address = env
        .storage()
        .instance()
        .get(&(KEY_USER_MGMT_ADDR,))
        .expect("user_mgmt_addr not configured");

    env.invoke_contract(
        &user_mgmt_addr,
        &Symbol::new(&env, "is_admin"),
        (who.clone(),).into_val(&env),
    )
}

/// Require that the caller has access rights to the profile
/// Access is granted if:
/// - The caller is the profile owner
/// - The caller is an admin
pub fn require_profile_access(env: &Env, caller: &Address, profile_owner: &Address) {
    caller.require_auth();
    
    if caller != profile_owner && !is_admin(env, caller) {
        panic!("{}", Error::Unauthorized.to_string());
    }
}

/// Require that the caller is an admin
pub fn require_admin(env: &Env, caller: &Address) {
    caller.require_auth();
    
    if !is_admin(env, caller) {
        panic!("{}", Error::Unauthorized.to_string());
    }
}