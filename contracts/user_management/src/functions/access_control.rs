// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{AdminConfig, DataKey};
use soroban_sdk::{Address, Env, Vec};

/// Check if the system is initialized
pub fn is_system_initialized(env: &Env) -> bool {
    if let Some(config) = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
    {
        config.initialized
    } else {
        false
    }
}

/// Check if a user is the super admin
pub fn is_super_admin(env: &Env, who: &Address) -> bool {
    if let Some(config) = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
    {
        config.super_admin == *who
    } else {
        false
    }
}

/// Check if a user is a regular admin
pub fn is_regular_admin(env: &Env, who: &Address) -> bool {
    let admins: Option<Vec<Address>> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins);
    
    match admins {
        Some(list) => list.iter().any(|a| a == *who),
        None => false,
    }
}

/// Check if a user has any admin privileges (super admin or regular admin)
pub fn is_admin(env: &Env, who: &Address) -> bool {
    is_super_admin(env, who) || is_regular_admin(env, who)
}

/// Require that the caller is the super admin
pub fn require_super_admin(env: &Env, caller: &Address) {
    caller.require_auth();
    
    if !is_super_admin(env, caller) {
        handle_error(env, Error::AccessDenied);
    }
}

/// Require that the caller has admin privileges
pub fn require_admin(env: &Env, caller: &Address) {
    caller.require_auth();
    
    if !is_admin(env, caller) {
        handle_error(env, Error::AccessDenied);
    }
}

/// Require that the system is initialized
pub fn require_initialized(env: &Env) {
    if !is_system_initialized(env) {
        handle_error(env, Error::SystemNotInitialized);
    }
}

/// Require that the caller is authorized to manage the target address
/// Authorization is granted if:
/// - The caller is the target address
/// - The caller is a super admin
/// - The caller is an admin and the target is not an admin
pub fn require_user_management_auth(env: &Env, caller: &Address, target: &Address) {
    caller.require_auth();
    
    if caller == target {
        return;
    }
    
    if is_super_admin(env, caller) {
        return;
    }
    
    if is_regular_admin(env, caller) && !is_admin(env, target) {
        return;
    }
    
    handle_error(env, Error::AccessDenied);
}

/// Check if the caller has admin privileges (including super admin)    

pub fn require_admin(env: &Env, caller: &Address) {    if !is_admin {

    // Require authentication from the caller        handle_error(env, Error::AccessDenied);

    caller.require_auth();    }

}

    // Get admin configuration

    let admin_config = envpub fn require_super_admin(env: &Env, caller: &Address) {

        .storage()    // Require authentication from the caller

        .persistent()    caller.require_auth();

        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)

        .unwrap_or_else(|| handle_error(env, Error::SystemNotInitialized));    // Get admin configuration

    let admin_config = env

    // Check if caller is super admin        .storage()

    if admin_config.super_admin == *caller {        .persistent()

        return;        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)

    }        .unwrap_or_else(|| handle_error(env, Error::SystemNotInitialized));



    // Check if caller is in admin list    // Check if caller is super admin

    let is_admin = env.storage().persistent().has(&DataKey::Admin(caller.clone()));    if admin_config.super_admin != *caller {

            handle_error(env, Error::AccessDenied);

    if !is_admin {    }

        handle_error(env, Error::AccessDenied);}

    }

}pub fn require_self_or_admin(env: &Env, caller: &Address, target: &Address) {

    // If caller is the target, just require their auth

/// Check if the caller is either the target user or an admin    if caller == target {

pub fn require_self_or_admin(env: &Env, caller: &Address, target: &Address) {        caller.require_auth();

    // If caller is the target, just require their auth        return;

    if caller == target {    }

        caller.require_auth();

        return;    // Otherwise, require admin privileges

    }    require_admin(env, caller);

}
    // Otherwise, require admin privileges
    require_admin(env, caller);
}

/// Check if the caller is the target user
pub fn require_self(env: &Env, caller: &Address, target: &Address) {
    caller.require_auth();
    
    if caller != target {
        handle_error(env, Error::AccessDenied);
    }
}