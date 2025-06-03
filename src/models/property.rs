use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

// --- Property Model (Database Representation) ---
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Property {
    pub id: Uuid,
    pub address: String,
    pub unit_number: Option<String>,
    pub current_rent_amount: BigDecimal,
    pub current_tenant_id: Option<Uuid>, // Foreign key to the users table
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- Property DTOs ---

// For creating a new property (API Request Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProperty {
    pub address: String,
    pub unit_number: Option<String>,
    pub current_rent_amount: BigDecimal,
    pub current_tenant_id: Option<Uuid>,
}
