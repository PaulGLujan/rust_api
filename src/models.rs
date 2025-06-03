use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// --- User Model ---
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Payment Model ---
// Make sure this matches your payment_status ENUM in the database
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Overdue,
    PartiallyPaid
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: sqlx::types::Decimal, // Use sqlx::types::Decimal for DECIMAL(10, 2)
    pub currency: String,
    pub status: PaymentStatus,
    pub description: Option<String>, // Option<String> because TEXT is nullable
    pub transaction_id: Option<String>, // Option<String> because TEXT is nullable and UNIQUE
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Request/Response DTOs (Data Transfer Objects) ---
// These are separate from database models and represent data going into/out of APIs

// For user registration
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

// For user login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

// For creating a new payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePayment {
    pub amount: sqlx::types::Decimal,
    pub currency: String,
    pub description: Option<String>,
}

// For a simplified successful login response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub username: String,
    pub token: String,
}

// For a simplified payment response (could be the full payment struct)
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: sqlx::types::Decimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub description: Option<String>,
}