use crate::AppState;
use axum::extract::{Json, State};

// Import your custom error and models
use crate::errors::AppError;
use crate::models::property::{CreateProperty, Property};

// Import database operations
use crate::db;

/// Handles creation of a new property.
pub async fn create_property(
    State(app_state): State<AppState>,
    Json(new_property): Json<CreateProperty>,
) -> Result<Json<Property>, AppError> {
    let property = db::create_property(&app_state.pool, new_property).await?;
    Ok(Json(property))
}

/// Handles listing all properties.
pub async fn list_properties(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Property>>, AppError> {
    let properties = db::list_properties(&app_state.pool).await?;
    Ok(Json(properties))
}
