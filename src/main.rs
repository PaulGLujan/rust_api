mod db;
mod errors;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // For logging

use tokio::net::TcpListener;

// Import all your handler functions
// Thanks to src/handlers/mod.rs, you can import them all directly.
use crate::handlers::{
    create_payment, create_property, list_payments, list_properties, login_user, register_user,
};

#[derive(Debug, Clone)]
pub struct JwtSecret(String);

impl From<String> for JwtSecret {
    fn from(secret: String) -> Self {
        JwtSecret(secret)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: JwtSecret,
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // Initialize Tracing for Logging (Optional but Recommended)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file (for local development)
    dotenv().ok();

    // Get DATABASE_URL and JWT_SECRET from environment variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("FATAL: DATABASE_URL must be set in .env or environment");
    let jwt_secret_string =
        std::env::var("JWT_SECRET").expect("FATAL: JWT_SECRET must be set in .env or environment");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Limit max connections for performance
        .connect(&database_url)
        .await
        .expect("FATAL: Failed to connect to Postgres database.");

    // Initialize the JwtSecret struct to be passed in Axum State
    let jwt_secret = JwtSecret(jwt_secret_string);

    // Create the combined application state
    let app_state = AppState {
        pool: pool,
        jwt_secret,
    };

    // Define the routes and attach handlers
    let app = Router::new()
        .route("/health_check", get(health_check))
        // User routes
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        // Property routes
        .route("/properties", post(create_property).get(list_properties))
        // Payment routes
        .route("/payments", post(create_payment).get(list_payments))
        // Note: For now, these routes are open. We'll add authentication middleware later.
        // Add the database pool and JWT secret to the application state
        .with_state(app_state);

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    // Start the server
    let listener = TcpListener::bind(&addr)
        .await
        .expect("FATAL: Failed to bind address");
    axum::serve(listener, app) // Use axum::serve
        .await
        .expect("FATAL: Server failed to start");
}
