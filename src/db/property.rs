use crate::errors::AppError;
use crate::models::{CreateProperty, Property};
use sqlx::PgPool;

/// Creates a new property in the database.
pub async fn create_property(
    pool: &PgPool,
    new_property: CreateProperty,
) -> Result<Property, AppError> {
    let property = sqlx::query_as!(
        Property,
        r#"
        INSERT INTO properties (address, unit_number, current_rent_amount, current_tenant_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, address, unit_number, current_rent_amount, current_tenant_id, created_at, updated_at
        "#,
        new_property.address,
        new_property.unit_number,
        new_property.current_rent_amount,
        new_property.current_tenant_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to create property: {}", e)))?;

    Ok(property)
}

/// Lists all properties in the database.
pub async fn list_properties(pool: &PgPool) -> Result<Vec<Property>, AppError> {
    let properties = sqlx::query_as!(
        Property,
        r#"
        SELECT id, address, unit_number, current_rent_amount, current_tenant_id, created_at, updated_at
        FROM properties
        ORDER BY address, unit_number
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to list properties: {}", e)))?;

    Ok(properties)
}
