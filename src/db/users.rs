use crate::errors::AppError;
use crate::models::{RegisterUser, User};
use sqlx::PgPool;

/// Creates a new user in the database.
pub async fn create_user(
    pool: &PgPool,
    new_user: RegisterUser,
    password_hash: String,
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, password_hash, created_at, updated_at
        "#,
        new_user.username,
        password_hash,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::Conflict("Username already taken".into());
            }
        }
        AppError::InternalServerError(format!("Failed to create user: {}", e))
    })?;

    Ok(user)
}

/// Finds a user by their username.
pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to find user: {}", e)))?;

    Ok(user)
}
