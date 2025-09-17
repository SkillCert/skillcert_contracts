// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{Course, DataKey};
use soroban_sdk::{Address, Env, String, Symbol};

const KEY_USER_MGMT_ADDR: &str = "user_mgmt_addr";

/// Checks if a user is an admin by querying the user_management contract
pub fn is_admin(env: &Env, who: &Address) -> bool {
    // Get user_management contract address
    let user_mgmt_addr: Address = env
        .storage()
        .instance()
        .get(&(KEY_USER_MGMT_ADDR,))
        .expect("user_mgmt_addr not configured; call initialize/set_config");

    // Cross-contract call to check admin status
    env.invoke_contract(
        &user_mgmt_addr,
        &Symbol::new(&env, "is_admin"),
        (who.clone(),).into_val(&env),
    )
}

/// Require that the caller is a course creator
pub fn require_course_creator(env: &Env, caller: &Address, course_id: &String) {
    // Require authentication from the caller
    caller.require_auth();

    // Get course data
    let course = env
        .storage()
        .persistent()
        .get::<DataKey, Course>(&DataKey::Course(course_id.clone()))
        .unwrap_or_else(|| handle_error(env, Error::CourseNotFound));

    // Check if caller is the course creator
    if course.creator != *caller {
        handle_error(env, Error::Unauthorized);
    }
}

/// Require that the caller is either a course creator or an admin
pub fn require_course_creator_or_admin(env: &Env, caller: &Address, course_id: &String) {
    // Require authentication from the caller
    caller.require_auth();

    // Get course data
    let course = env
        .storage()
        .persistent()
        .get::<DataKey, Course>(&DataKey::Course(course_id.clone()))
        .unwrap_or_else(|| handle_error(env, Error::CourseNotFound));

    // Allow if caller is course creator
    if course.creator == *caller {
        return;
    }

    // If not creator, check if admin
    if !is_admin(env, caller) {
        handle_error(env, Error::Unauthorized);
    }
}

/// Require that the caller is an admin
pub fn require_admin(env: &Env, caller: &Address) {
    caller.require_auth();
    
    if !is_admin(env, caller) {
        handle_error(env, Error::Unauthorized);
    }
}

/// Helper function to check if the caller is a course creator
pub fn is_course_creator(env: &Env, caller: &Address, course_id: &String) -> bool {
    if let Some(course) = env
        .storage()
        .persistent()
        .get::<DataKey, Course>(&DataKey::Course(course_id.clone()))
    {
        course.creator == *caller
    } else {
        false
    }
}

/// Helper function to check course existence
pub fn check_course_exists(env: &Env, course_id: &String) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Course(course_id.clone()))
}

/// Check if the caller is either the course creator or an admin    // Require authentication from the caller

pub fn require_course_creator_or_admin(env: &Env, caller: &Address, course_id: &String) {    caller.require_auth();

    // Require authentication from the caller

    caller.require_auth();    // Get course data

    let course = env

    // Get course data        .storage()

    let course = env        .persistent()

        .storage()        .get::<DataKey, Course>(&DataKey::Course(course_id.clone()))

        .persistent()        .unwrap_or_else(|| handle_error(env, Error::CourseNotFound));

        .get::<DataKey, Course>(&DataKey::Course(course_id.clone()))

        .unwrap_or_else(|| handle_error(env, Error::CourseNotFound));    // Check if caller is the course creator

    if course.creator == *caller {

    // Check if caller is the course creator        return;

    if course.creator == *caller {    }

        return;

    }    // If not the creator, check if they're an admin

    let user_management = env.storage().instance().get(&DataKey::UserManagementContract);

    // If not creator, check if they're an admin through user management contract    if let Some(user_mgmt_id) = user_management {

    let user_management = env.storage().instance().get(&DataKey::UserManagementContract);        // Check admin status through user management contract

    if let Some(user_mgmt_id) = user_management {        let client = crate::UserManagementClient::new(env, &user_mgmt_id);

        let client = crate::UserManagementClient::new(env, &user_mgmt_id);        if client.is_admin(caller) {

        if client.is_admin(caller) {            return;

            return;        }

        }    }

    }

    handle_error(env, Error::AccessDenied);

    handle_error(env, Error::Unauthorized);}
}