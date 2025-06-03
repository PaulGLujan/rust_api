use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

// Import your custom error and models
use crate::errors::AppError;
use crate::models::property::{CreateProperty, Property};

// Import database operations
use crate::db;

/// Handles creation of a new property.
pub async fn create_property(
    State(pool): State<PgPool>,
    Json(new_property): Json<CreateProperty>,
) -> Result<Json<Property>, AppError> {
    let property = db::create_property(&pool, new_property).await?;
    Ok(Json(property))
}

/// Handles listing all properties.
pub async fn list_properties(State(pool): State<PgPool>) -> Result<Json<Vec<Property>>, AppError> {
    let properties = db::list_properties(&pool).await?;
    Ok(Json(properties))
}
