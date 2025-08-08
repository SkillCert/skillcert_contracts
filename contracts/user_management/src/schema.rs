use soroban_sdk::{Address, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub profession: Option<String>,
    pub goals: Option<String>,
    pub country: String,
    pub user: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    UserProfile(Address), // This represents the ("user_profile", user_address) key
} 