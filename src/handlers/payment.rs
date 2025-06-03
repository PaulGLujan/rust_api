// src/handlers/payment.rs

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

// Import your custom error and models
use crate::errors::AppError;
use crate::models::payment::{CreatePayment, Payment, PaymentResponse, PaymentStatus};

// Import database operations
use crate::db;

/// Handles creation of a new payment.
pub async fn create_payment(
    State(pool): State<PgPool>,
    Json(new_payment): Json<CreatePayment>,
) -> Result<Json<PaymentResponse>, AppError> {
    // For now, new payments start as Pending.
    // In a real app, this might involve a payment gateway.
    let initial_status = PaymentStatus::Pending;
    let transaction_id = None; // Placeholder for external transaction ID

    let payment = db::create_payment(&pool, new_payment, initial_status, transaction_id).await?;

    // Convert the created Payment DB model to a PaymentResponse DTO
    Ok(Json(PaymentResponse {
        id: payment.id,
        user_id: payment.user_id,
        property_id: payment.property_id,
        amount: payment.amount,
        currency: payment.currency,
        status: payment.status,
        notes: payment.notes,
        transaction_id: payment.transaction_id,
        due_date: payment.due_date,
        period_start: payment.period_start,
        period_end: payment.period_end,
    }))
}

/// Handles listing payments with optional filters.
pub async fn list_payments(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Payment>>, AppError> {
    let user_id_filter = params.get("user_id").and_then(|s| s.parse::<Uuid>().ok());
    let property_id_filter = params
        .get("property_id")
        .and_then(|s| s.parse::<Uuid>().ok());

    let payments = db::list_payments(&pool, user_id_filter, property_id_filter).await?;
    Ok(Json(payments))
}
