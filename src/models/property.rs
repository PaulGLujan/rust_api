use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Decimal}; // Use sqlx::types::Decimal for DECIMAL
use uuid::Uuid;

// --- Property Model (Database Representation) ---
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Property {
    pub id: Uuid,
    pub address: String,
    pub unit_number: Option<String>,
    pub current_rent_amount: Decimal,
    pub current_tenant_id: Option<Uuid>, // Foreign key to the users table
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Property DTOs ---

// For creating a new property (API Request Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProperty {
    pub address: String,
    pub unit_number: Option<String>,
    pub current_rent_amount: Decimal,
    pub current_tenant_id: Option<Uuid>,
}
