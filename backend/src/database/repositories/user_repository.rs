use crate::database::models::User;
use crate::database::connection::DbPool;
use sqlx::{Error as SqlxError};

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        pool: &DbPool,
        username: &str,
        email: &str,
        wallet_address: &str,
        user_type: &str,
        organization: Option<&str>,
        bio: Option<&str>,
    ) -> Result<User, SqlxError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, email, wallet_address, user_type, organization, bio, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id, username, email, wallet_address, user_type, organization, bio, created_at
            "#,
            username,
            email,
            wallet_address,
            user_type,
            organization,
            bio
        )
        .fetch_one(pool)
        .await?;

        Ok(User {
            id: row.id,
            username: row.username,
            email: row.email,
            wallet_address: row.wallet_address,
            user_type: row.user_type,
            organization: row.organization,
            bio: row.bio,
            created_at: row.created_at,
        })
    }

    pub async fn find_by_wallet_address(
        pool: &DbPool,
        wallet_address: &str,
    ) -> Result<Option<User>, SqlxError> {
        let row = sqlx::query!(
            "SELECT id, username, email, wallet_address, user_type, organization, bio, created_at FROM users WHERE wallet_address = $1",
            wallet_address
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                wallet_address: row.wallet_address,
                user_type: row.user_type,
                organization: row.organization,
                bio: row.bio,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_email(
        pool: &DbPool,
        email: &str,
    ) -> Result<Option<User>, SqlxError> {
        let row = sqlx::query!(
            "SELECT id, username, email, wallet_address, user_type, organization, bio, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                wallet_address: row.wallet_address,
                user_type: row.user_type,
                organization: row.organization,
                bio: row.bio,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_id(
        pool: &DbPool,
        user_id: i32,
    ) -> Result<Option<User>, SqlxError> {
        let row = sqlx::query!(
            "SELECT id, username, email, wallet_address, user_type, organization, bio, created_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                wallet_address: row.wallet_address,
                user_type: row.user_type,
                organization: row.organization,
                bio: row.bio,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }
}