use crate::database::connection::DbPool;
use crate::database::repositories::UserRepository;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use tracing::{info, error};
use shared::dto::{SignUpRequest, SignUpResponse, UserPublic, UserType};
use crate::database::models::User;

fn create_user_public(user: &User) -> UserPublic {
    let user_type = match user.user_type.as_str() {
        "Ambassador" => UserType::Ambassador,
        "ChapterLead" => UserType::ChapterLead,
        _ => UserType::Ambassador, // default fallback
    };

    UserPublic {
        id: user.id.to_string(),
        username: user.username.clone(),
        email: user.email.clone(),
        wallet_address: user.wallet_address.clone(),
        user_type,
        organization: user.organization.clone(),
        bio: user.bio.clone(),
        created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
    }
}

fn create_error_user_public() -> UserPublic {
    UserPublic {
        id: "error".to_string(),
        username: "Error".to_string(),
        email: "".to_string(),
        wallet_address: "".to_string(),
        user_type: UserType::Ambassador,
        organization: None,
        bio: None,
        created_at: "".to_string(),
    }
}

pub async fn signup(
    State(pool): State<DbPool>,
    Json(req): Json<SignUpRequest>,
) -> (StatusCode, Json<SignUpResponse>) {
    println!("🚀 NEW SIGNUP REQUEST");
    println!("   Username: {}", req.username);
    println!("   Email: {}", req.email);
    println!("   Wallet Address: {}", req.wallet_address);
    println!("   User Type: {:?}", req.user_type);
    println!("   Organization: {:?}", req.organization);
    println!("   Bio: {:?}", req.bio);
    println!("   ────────────────────────────────────");

    info!("Received signup request: username={}, email={}, wallet_address={}, user_type={:?}",
          req.username, req.email, req.wallet_address, req.user_type);

    // Check if user already exists by email or wallet address
    match UserRepository::find_by_email(&pool, &req.email).await {
        Ok(Some(_)) => {
            println!("❌ SIGNUP FAILED: Email already exists ({})", req.email);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: "User with this email already exists".to_string(),
            };
            return (StatusCode::CONFLICT, Json(resp));
        }
        Ok(None) => {
            // Continue with wallet address check
        }
        Err(e) => {
            println!("❌ SIGNUP FAILED: Database error checking email: {}", e);
            error!("Database error checking email: {:?}", e);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: format!("Database error: {}", e),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
        }
    }

    match UserRepository::find_by_wallet_address(&pool, &req.wallet_address).await {
        Ok(Some(_)) => {
            println!("❌ SIGNUP FAILED: Wallet address already exists ({})", req.wallet_address);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: "User with this wallet address already exists".to_string(),
            };
            (StatusCode::CONFLICT, Json(resp))
        }
        Ok(None) => {
            let user_type_str = req.user_type.to_string();
            match UserRepository::create_user(
                &pool,
                &req.username,
                &req.email,
                &req.wallet_address,
                &user_type_str,
                req.organization.as_deref(),
                req.bio.as_deref(),
            ).await {
                Ok(db_user) => {
                    println!("✅ SIGNUP SUCCESS!");
                    println!("   New User ID: {}", db_user.id);
                    println!("   Username: {}", db_user.username);
                    println!("   Email: {}", db_user.email);
                    println!("   User Type: {}", db_user.user_type);
                    println!("   🎉 Welcome to Stellar Europe!");
                    println!("   ════════════════════════════════════");

                    let user_public = create_user_public(&db_user);

                    let resp = SignUpResponse {
                        user: user_public,
                        message: "User created successfully!".to_string(),
                    };
                    (StatusCode::CREATED, Json(resp))
                }
                Err(e) => {
                    println!("❌ SIGNUP FAILED: Database error creating user: {}", e);
                    error!("Database error creating user: {:?}", e);
                    let resp = SignUpResponse {
                        user: create_error_user_public(),
                        message: format!("Failed to create user: {}", e),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
                }
            }
        }
        Err(e) => {
            println!("❌ SIGNUP FAILED: Database error finding user: {}", e);
            error!("Database error finding user: {:?}", e);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: format!("Database error: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
}