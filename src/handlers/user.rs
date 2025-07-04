use axum::extract::{Json, State};
use uuid::Uuid;

// For password hashing
use bcrypt::{hash, verify};

// For JWT
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::models::user::{AuthResponse, LoginUser, RegisterUser, User};

use crate::AppState;
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid, // Subject (user ID)
    username: String,
    exp: usize, // Expiration time
}

/// Handles user registration.
pub async fn register_user(
    State(app_state): State<AppState>,
    Json(new_user): Json<RegisterUser>,
) -> Result<Json<User>, AppError> {
    // Hash the password
    let hashed_password = hash(&new_user.password, 10)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?;

    // Create user in DB
    let user = db::create_user(&app_state.pool, new_user, hashed_password).await?;

    Ok(Json(user))
}

/// Handles user login and JWT generation.
pub async fn login_user(
    State(app_state): State<AppState>,
    Json(login_data): Json<LoginUser>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user by username
    let user = db::find_user_by_username(&app_state.pool, &login_data.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid username or password".into()))?;

    // Verify password
    let passwords_match = verify(&login_data.password, &user.password_hash).map_err(|e| {
        AppError::InternalServerError(format!("Password verification error: {}", e))
    })?;

    if !passwords_match {
        return Err(AppError::Unauthorized(
            "Invalid username or password".into(),
        ));
    }

    // Generate JWT token
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token valid for 24 hours
        .ok_or_else(|| {
            AppError::InternalServerError("Failed to calculate token expiration".into())
        })?
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        exp: expiration_time,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&app_state.jwt_secret.0.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Failed to generate JWT: {}", e)))?;

    Ok(Json(AuthResponse {
        user_id: user.id,
        username: user.username,
        token,
    }))
}
