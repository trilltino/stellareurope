use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub wallet_address: String,
    pub user_type: String, // "Ambassador" or "ChapterLead"
    pub organization: Option<String>,
    pub bio: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new_with_type(
        username: String,
        email: String,
        wallet_address: String,
        user_type: String,
        organization: Option<String>,
        bio: Option<String>
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            username,
            email,
            wallet_address,
            user_type,
            organization,
            bio,
            created_at: Some(now),
        }
    }
}