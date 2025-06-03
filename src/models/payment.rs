use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

// --- Payment ENUM (Database Representation) ---
#[derive(Clone, Debug, PartialEq, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Overdue,
    PartiallyPaid,
}

// --- Payment Model (Database Representation) ---
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub property_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub notes: Option<String>,
    pub transaction_id: Option<String>,
    pub due_date: Option<Date>,
    pub period_start: Option<Date>,
    pub period_end: Option<Date>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- Payment DTOs ---

// For creating a new payment request (API Request Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePayment {
    pub user_id: Uuid,
    pub property_id: Uuid,
    pub amount: BigDecimal,
    pub currency: String,
    pub notes: Option<String>,
    pub due_date: Option<Date>,
    pub period_start: Option<Date>,
    pub period_end: Option<Date>,
}

// For a simplified payment response (API Response Body)
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub property_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub notes: Option<String>,
    pub transaction_id: Option<String>,
    pub due_date: Option<Date>,
    pub period_start: Option<Date>,
    pub period_end: Option<Date>,
}
