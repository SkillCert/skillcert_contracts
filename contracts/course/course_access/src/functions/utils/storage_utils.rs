// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, String, Vec, symbol_short};

use crate::error::{handle_error, Error};
use crate::schema::{CourseAccess, CourseUsers, UserCourses};
use crate::functions::config::{TTL_BUMP, TTL_TTL};

const TEMP_TTL: u32 = 900; // 15 minutes

/// Brief description: Retrieves or creates the user's courses.
///
/// # Arguments
///
/// * `env` - The environment context.
/// * `user` - The address of the user.
///
/// # Returns
///
/// * `UserCourses` - The user's courses, either retrieved from cache or created.
pub fn get_or_create_user_courses(
    env: &Env,
    user: &Address,
) -> UserCourses {
    let temp_key = (symbol_short!("temp_user_courses"), user.clone());
    
    // Try cache first
    if let Some(courses) = env.storage().temporary().get(&temp_key) {
        return courses;
    }

    let user_courses: UserCourses = env
        .storage()
        .persistent()
        .get(&(symbol_short!("user_courses"), user.clone()))
        .unwrap_or_else(|| UserCourses {
            user: user.clone(),
            courses: Vec::new(env),
        });

    // Cache result
    env.storage().temporary().set(&temp_key, &user_courses);
    env.storage().temporary().extend_ttl(&temp_key, 0, TEMP_TTL);

    user_courses
}

/// Brief description: Retrieves or creates the list of users for a given course.
///
/// # Arguments
///
/// * `env` - The environment context.
/// * `course_id` - The ID of the course.
///
/// # Returns
///
/// * `CourseUsers` - The users enrolled in the course, either retrieved from cache or created.
pub fn get_or_create_course_users(
    env: &Env,
    course_id: &String,
) -> CourseUsers {
    let temp_key = (symbol_short!("temp_course_users"), course_id.clone());
    
    // Try cache first
    if let Some(users) = env.storage().temporary().get(&temp_key) {
        return users;
    }

    let course_users: CourseUsers = env
        .storage()
        .persistent()
        .get(&(symbol_short!("course_users"), course_id.clone()))
        .unwrap_or_else(|| CourseUsers {
            course: course_id.clone(),
            users: Vec::new(env),
        });


    env.storage().temporary().set(&temp_key, &course_users);
    env.storage().temporary().extend_ttl(&temp_key, 0, TEMP_TTL);

    course_users
}

/// Brief description: Updates access mappings for a user in a course.
///
/// # Arguments
///
/// * `env` - The environment context.
/// * `course_id` - The ID of the course.
/// * `user` - The address of the user.
/// * `add` - A boolean flag indicating whether to grant (true) or revoke (false) access.
///
/// # Returns
///
/// * `()` - This function does not return a value.
pub fn update_access_mappings(
    env: &Env,
    course_id: &String,
    user: &Address,
    add: bool, // true for grant, false for revoke
) {
    let mut user_courses = get_or_create_user_courses(env, user);
    let mut course_users = get_or_create_course_users(env, course_id);

    if add {
        if !user_courses.courses.contains(course_id) {
            user_courses.courses.push_back(course_id.clone());
        }
        if !course_users.users.contains(user) {
            course_users.users.push_back(user.clone());
        }
    } else {
        user_courses.courses.retain(|c| c != course_id);
        course_users.users.retain(|u| u != user);
    }

    // Update persistent storage
    let user_courses_key = (symbol_short!("user_courses"), user.clone());
    let course_users_key = (symbol_short!("course_users"), course_id.clone());

    env.storage().persistent().set(&user_courses_key, &user_courses);
    env.storage().persistent().set(&course_users_key, &course_users);
    
    env.storage().persistent().extend_ttl(&user_courses_key, TTL_BUMP, TTL_TTL);
    env.storage().persistent().extend_ttl(&course_users_key, TTL_BUMP, TTL_TTL);

    // Update cache
    let temp_user_key = (symbol_short!("temp_user_courses"), user.clone());
    let temp_course_key = (symbol_short!("temp_course_users"), course_id.clone());

    env.storage().temporary().set(&temp_user_key, &user_courses);
    env.storage().temporary().set(&temp_course_key, &course_users);
}

/// Brief description: Checks if a user has access to a course.
///
/// # Arguments
///
/// * `env` - The environment context.
/// * `course_id` - The ID of the course.
/// * `user` - The address of the user.
///
/// # Returns
///
/// * `bool` - True if the user has access to the course, otherwise false.
pub fn has_course_access(
    env: &Env,
    course_id: &String,
    user: &Address,
) -> bool {
    let temp_key = (
        symbol_short!("temp_access"),
        (course_id.clone(), user.clone()),
    );

    // Try cache first
    if let Some(has_access) = env.storage().temporary().get(&temp_key) {
        return has_access;
    }

    // Check persistent storage
    let has_access = env
        .storage()
        .persistent()
        .has(&(symbol_short!("course_access"), (course_id.clone(), user.clone())));

    // Cache result
    env.storage().temporary().set(&temp_key, &has_access);
    env.storage().temporary().extend_ttl(&temp_key, 0, TEMP_TTL);

    has_access
}

/// Brief description: Invalidates the cache for a specific course's users.
///
/// # Arguments
///
/// * `env` - The environment context.
/// * `course_id` - The ID of the course.
///
/// # Returns
///
/// * `()` - This function does not return a value.
pub fn invalidate_course_access_cache(
    env: &Env,
    course_id: &String,
) {
    let temp_users_key = (symbol_short!("temp_course_users"), course_id.clone());
    env.storage().temporary().remove(&temp_users_key);
}

pub fn invalidate_user_access_cache(
    env: &Env,
    user: &Address,
) {
    let temp_courses_key = (symbol_short!("temp_user_courses"), user.clone());
    env.storage().temporary().remove(&temp_courses_key);
}
