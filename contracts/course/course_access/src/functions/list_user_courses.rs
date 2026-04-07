// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, Vec};

use crate::schema::{DataKey, UserCourses};

/// Retrieves the list of courses associated with a specific user.
///
/// This function queries the persistent storage to fetch all courses 
/// that the specified user has access to. If no courses are found for
/// the user, it returns an empty `UserCourses` struct.
///
/// # Arguments
///
/// * `env` - The Soroban environment context for accessing storage and other SDK features.
/// * `user` - The blockchain address of the user whose courses are being retrieved.
///
/// # Returns
///
/// * `UserCourses` - A struct containing the user's address and a vector of their course IDs.
///   If the user has no courses, returns a `UserCourses` with an empty courses vector.
pub fn list_user_courses(env: Env, user: Address) -> UserCourses {
    let key: DataKey = DataKey::UserCourses(user.clone());
    let res: UserCourses = env.storage().persistent().get(&key).unwrap_or(UserCourses {
        user: user.clone(),
        courses: Vec::new(&env),
    });

    return res
}

#[cfg(test)]
mod test {
    use crate::schema::DataKey;
    use crate::{CourseAccessContract, UserCourses};
    use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};
    use super::list_user_courses;

    /// Tests the `list_user_courses` function to verify correct retrieval of user courses.
    ///
    /// This test sets up a mock environment with a registered contract and user,
    /// creates course data, stores it in persistent storage, and verifies that
    /// `list_user_courses` correctly retrieves the stored data.
    #[test]
    fn test_list_user_courses() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseAccessContract, {});
        let user: Address = Address::generate(&env);
        let key: DataKey = DataKey::UserCourses(user.clone());
        let course_id: String = String::from_str(&env, "test_course_123");
        let courses: soroban_sdk::Vec<String> = vec![&env, course_id];
        let user_courses: UserCourses = UserCourses {
            user: user.clone(),
            courses: courses,
        };
        
        // Set up initial course data and perform test within contract context
        env.clone().as_contract(&contract_id, || {
            env.storage().persistent().set(&key, &user_courses);
            let result: UserCourses = list_user_courses(env, user.clone());
            assert_eq!(result, user_courses);
        });
    }
}
