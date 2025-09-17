// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::DataKey;
use soroban_sdk::{Address, Env, String, Symbol};

const KEY_USER_MGMT_ADDR: &str = "user_mgmt_addr";
const KEY_COURSE_REG_ADDR: &str = "course_reg_addr";

/// Check if a user has admin privileges
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

/// Check if a user is the creator of a course
pub fn is_course_creator(env: &Env, course_id: &String, who: &Address) -> bool {
    let course_reg_addr: Address = env
        .storage()
        .instance()
        .get(&(KEY_COURSE_REG_ADDR,))
        .expect("course_reg_addr not configured");

    env.invoke_contract(
        &course_reg_addr,
        &Symbol::new(&env, "is_course_creator"),
        (course_id.clone(), who.clone()).into_val(&env),
    )
}

/// Require that the caller has access to the course
pub fn require_course_access(env: &Env, caller: &Address, course_id: &String) {
    caller.require_auth();
    
    // Allow if caller has access or is admin
    if !has_access(env, caller, course_id) && !is_admin(env, caller) && !is_course_creator(env, course_id, caller) {
        handle_error(env, Error::Unauthorized);
    }
}

/// Check if a user has access to a course
pub fn has_access(env: &Env, user: &Address, course_id: &String) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::UserAccess(course_id.clone(), user.clone()))
}

/// Require that the caller has management rights (creator or admin)
pub fn require_management_rights(env: &Env, caller: &Address, course_id: &String) {
    caller.require_auth();
    
    if !is_course_creator(env, course_id, caller) && !is_admin(env, caller) {
        handle_error(env, Error::Unauthorized);
    }
}

    }

pub fn require_access_or_admin(env: &Env, caller: &Address, course_id: &String, target: &Address) {

    // Require authentication from the caller    handle_error(env, Error::AccessDenied);

    caller.require_auth();}



    // If caller is target, check if they have accesspub fn require_course_owner(env: &Env, caller: &Address, course_id: &String) {

    if caller == target {    // Require authentication from the caller

        let access_key = DataKey::CourseAccess(course_id.clone(), caller.clone());    caller.require_auth();

        if !env.storage().persistent().has(&access_key) {

            handle_error(env, Error::UserNoAccessCourse);    // Get current owner of the course access

        }    let owner = env

        return;        .storage()

    }        .persistent()

        .get::<DataKey, Address>(&DataKey::CourseOwner(course_id.clone()))

    // If not target, check if they're an admin through user management contract        .unwrap_or_else(|| handle_error(env, Error::CourseAccessNotFound));

    let user_management = env.storage().instance().get(&DataKey::UserManagementContract);

    if let Some(user_mgmt_id) = user_management {    if *caller != owner {

        let client = crate::UserManagementClient::new(env, &user_mgmt_id);        handle_error(env, Error::AccessDenied);

        if client.is_admin(caller) {    }

            return;}
        }
    }

    handle_error(env, Error::Unauthorized);
}