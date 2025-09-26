use serde::{Deserialize, Serialize};
use super::auth::UserType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub email: String,
    pub wallet_address: String,
    pub user_type: UserType,
    pub organization: Option<String>,
    pub bio: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignUpResponse {
    pub user: UserPublic,
    pub message: String,
}