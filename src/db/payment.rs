// src/db/payment.rs

use crate::errors::AppError;
use crate::models::{CreatePayment, Payment, PaymentStatus};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool};
use time::Date;
use uuid::Uuid;

/// Creates a new payment record.
pub async fn create_payment(
    pool: &PgPool,
    new_payment: CreatePayment,
    status: PaymentStatus,
    transaction_id: Option<String>,
) -> Result<Payment, AppError> {
    let payment = sqlx::query_as!(
        Payment,
        r#"
        INSERT INTO payments (
            user_id, property_id, amount, currency, status,
            notes, transaction_id, due_date, period_start, period_end
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            id, user_id, property_id, amount, currency, status as "status!: PaymentStatus",
            notes, transaction_id, due_date, period_start, period_end, created_at, updated_at
        "#,
        new_payment.user_id,
        new_payment.property_id,
        new_payment.amount,
        new_payment.currency,
        status as PaymentStatus,
        new_payment.notes,
        transaction_id,
        new_payment.due_date,
        new_payment.period_start,
        new_payment.period_end,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to create payment: {}", e)))?;

    Ok(payment)
}

/// Lists payments, potentially filtered by user_id or property_id.
pub async fn list_payments(
    pool: &PgPool,
    user_id: Option<Uuid>,
    property_id: Option<Uuid>,
) -> Result<Vec<Payment>, AppError> {
    let mut query = r#"
        SELECT
            id, user_id, property_id, amount, currency, status as "status!: PaymentStatus",
            notes, transaction_id, due_date, period_start, period_end, created_at, updated_at
        FROM payments
    "#
    .to_string();

    let mut conditions = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode + Send + Sync>> = Vec::new();
    let mut param_idx = 1;

    if let Some(uid) = user_id {
        conditions.push(format!("user_id = ${}", param_idx));
        params.push(Box::new(uid));
        param_idx += 1;
    }
    if let Some(pid) = property_id {
        conditions.push(format!("property_id = ${}", param_idx));
        params.push(Box::new(pid));
        param_idx += 1;
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY created_at DESC");

    let mut q = sqlx::query_as(&query);
    for param in params {
        q = q.bind(param);
    }

    let payments = q
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to list payments: {}", e)))?;

    Ok(payments)
}
