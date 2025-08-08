pub mod schema;
pub mod functions;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Env, String, Address};
use crate::schema::UserProfile;

#[contract]
pub struct UserManagement;

#[contractimpl]
impl UserManagement {
    pub fn save_profile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
        user: Address,
    ) -> UserProfile {
        functions::save_profile::user_management_save_profile(
            env,
            name,
            email,
            profession,
            goals,
            country,
            user,
        )
    }
    
    pub fn update_profile(
        env: Env,
        caller: Address,
        name: Option<String>,
        email: Option<String>,
        profession: Option<String>,
        goals: Option<String>,
        country: Option<String>,
    ) -> UserProfile {
        functions::update_user_profile::user_management_update_profile(
            env,
            caller,
            name,
            email,
            profession,
            goals,
            country,
        )
    }
} 