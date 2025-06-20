use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

// --- User Model (Database Representation) ---
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- User DTOs (Data Transfer Objects) ---

// For user registration (API Request Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

// For user login (API Request Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

// For a simplified successful login response (API Response Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub username: String,
    pub token: String,
}
