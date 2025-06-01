use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgPool};
use std::{env, error::Error, net::SocketAddr};
use tokio;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Item {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct CreateItem {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");

    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/items", get(get_items))
        .route("/items", post(create_item))
        .with_state(pool);

    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port_str).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_items(State(pool): State<PgPool>) -> Json<Vec<Item>> {
    let items = sqlx::query_as::<_, Item>("select id, name from items")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch items from database");
    Json(items)
}

async fn create_item(State(pool): State<PgPool>, Json(new_item): Json<CreateItem>) -> Json<Item> {
    let new_item =
        sqlx::query_as::<_, Item>("insert into items (name) values ($1) returning id, name")
            .bind(&new_item.name)
            .fetch_one(&pool)
            .await
            .expect("Failed to insert item into database");
    Json(new_item)
}
