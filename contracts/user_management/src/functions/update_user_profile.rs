use soroban_sdk::{Address, Env, String};
use crate::schema::{UserProfile, DataKey};

pub fn user_management_update_profile(
    env: Env,
    caller: Address,
    name: Option<String>,
    email: Option<String>,
    profession: Option<String>,
    goals: Option<String>,
    country: Option<String>,
) -> UserProfile {
    // Verify caller authorization - only the profile owner can update
    caller.require_auth();
    
    // Retrieve existing profile
    let storage_key = DataKey::UserProfile(caller.clone());
    let existing_profile: UserProfile = env.storage().persistent()
        .get(&storage_key)
        .unwrap_or_else(|| panic!("User profile error: Profile not found for user"));
    
    // Verify ownership - caller must be the profile owner
    if existing_profile.user != caller {
        panic!("User profile error: Only profile owner can update");
    }
    
    // Create updated profile with partial updates
    let updated_profile = UserProfile {
        name: name.unwrap_or(existing_profile.name),
        email: email.unwrap_or(existing_profile.email),
        profession: profession.or(existing_profile.profession),
        goals: goals.or(existing_profile.goals),
        country: country.unwrap_or(existing_profile.country),
        user: existing_profile.user, // Address cannot be changed
    };
    
    // Validate required fields are not empty
    if updated_profile.name.is_empty() {
        panic!("User profile error: Name cannot be empty");
    }
    
    if updated_profile.email.is_empty() {
        panic!("User profile error: Email cannot be empty");
    }
    
    if updated_profile.country.is_empty() {
        panic!("User profile error: Country cannot be empty");
    }
    
    // Store the updated profile
    env.storage().persistent().set(&storage_key, &updated_profile);
    
    // Emit profile updated event
    env.events().publish(
        ("UserManagement", String::from_str(&env, "ProfileUpdated")),
        (String::from_str(&env, "profile_updated"), caller.clone())
    );
    
    updated_profile
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Address, String, Env, testutils::Address as _};
    use crate::{UserManagement, UserManagementClient};
    
    #[test]
    fn test_update_profile_success() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        // First create a profile
        let initial_name = String::from_str(&env, "John Doe");
        let initial_email = String::from_str(&env, "john@example.com");
        let initial_country = String::from_str(&env, "United States");
        
        client.save_profile(
            &initial_name,
            &initial_email,
            &Some(String::from_str(&env, "Software Engineer")),
            &Some(String::from_str(&env, "Learn blockchain")),
            &initial_country,
            &user,
        );
        
        // Now update the profile
        let new_name = String::from_str(&env, "John Smith");
        let new_profession = String::from_str(&env, "Senior Developer");
        
        let updated_profile = client.update_profile(
            &user,
            &Some(new_name.clone()),
            &None, // Keep existing email
            &Some(new_profession.clone()),
            &None, // Keep existing goals
            &None, // Keep existing country
        );
        
        // Verify updates
        assert_eq!(updated_profile.name, new_name);
        assert_eq!(updated_profile.email, initial_email);
        assert_eq!(updated_profile.profession, Some(new_profession));
        assert_eq!(updated_profile.goals, Some(String::from_str(&env, "Learn blockchain")));
        assert_eq!(updated_profile.country, initial_country);
        assert_eq!(updated_profile.user, user);
    }
    
    #[test]
    fn test_update_profile_partial_update() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        // First create a profile
        let initial_name = String::from_str(&env, "Jane Doe");
        let initial_email = String::from_str(&env, "jane@example.com");
        let initial_country = String::from_str(&env, "Canada");
        
        client.save_profile(
            &initial_name,
            &initial_email,
            &None,
            &None,
            &initial_country,
            &user,
        );
        
        // Update only goals field
        let new_goals = String::from_str(&env, "Master Rust programming");
        
        let updated_profile = client.update_profile(
            &user,
            &None, // Keep existing name
            &None, // Keep existing email
            &None, // Keep existing profession (None)
            &Some(new_goals.clone()), // Update goals
            &None, // Keep existing country
        );
        
        // Verify only goals was updated
        assert_eq!(updated_profile.name, initial_name);
        assert_eq!(updated_profile.email, initial_email);
        assert_eq!(updated_profile.profession, None);
        assert_eq!(updated_profile.goals, Some(new_goals));
        assert_eq!(updated_profile.country, initial_country);
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Profile not found for user")]
    fn test_update_nonexistent_profile() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        client.update_profile(
            &user,
            &Some(String::from_str(&env, "Test Name")),
            &None,
            &None,
            &None,
            &None,
        );
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Name cannot be empty")]
    fn test_update_profile_empty_name() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        // First create a profile
        client.save_profile(
            &String::from_str(&env, "John Doe"),
            &String::from_str(&env, "john@example.com"),
            &None,
            &None,
            &String::from_str(&env, "United States"),
            &user,
        );
        
        // Try to update with empty name
        client.update_profile(
            &user,
            &Some(String::from_str(&env, "")), // Empty name
            &None,
            &None,
            &None,
            &None,
        );
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Email cannot be empty")]
    fn test_update_profile_empty_email() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        // First create a profile
        client.save_profile(
            &String::from_str(&env, "John Doe"),
            &String::from_str(&env, "john@example.com"),
            &None,
            &None,
            &String::from_str(&env, "United States"),
            &user,
        );
        
        // Try to update with empty email
        client.update_profile(
            &user,
            &None,
            &Some(String::from_str(&env, "")), // Empty email
            &None,
            &None,
            &None,
        );
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Country cannot be empty")]
    fn test_update_profile_empty_country() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();
        
        // First create a profile
        client.save_profile(
            &String::from_str(&env, "John Doe"),
            &String::from_str(&env, "john@example.com"),
            &None,
            &None,
            &String::from_str(&env, "United States"),
            &user,
        );
        
        // Try to update with empty country
        client.update_profile(
            &user,
            &None,
            &None,
            &None,
            &None,
            &Some(String::from_str(&env, "")), // Empty country
        );
    }
}