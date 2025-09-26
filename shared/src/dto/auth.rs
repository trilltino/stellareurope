use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum UserType {
    Ambassador,
    ChapterLead,
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Ambassador => write!(f, "Ambassador"),
            UserType::ChapterLead => write!(f, "ChapterLead"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub wallet_address: String,
    pub user_type: UserType,
    pub organization: Option<String>,
    pub bio: Option<String>,
}