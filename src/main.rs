use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    name: String,
}

type SharedState = Arc<Mutex<Vec<Item>>>;

#[tokio::main]
async fn main() {
    let items = Arc::new(Mutex::new(Vec::<Item>::new()));

    let app = Router::new()
        .route("/items", get(get_items))
        .route("/items", post(create_item))
        .with_state(items);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_items(State(state): State<SharedState>) -> Json<Vec<Item>> {
    let items_guard = state.lock().unwrap();
    let items_cloned = items_guard.clone();
    Json(items_cloned)
}
async fn create_item(State(state): State<SharedState>, Json(new_item): Json<Item>) -> Json<Item> {
    let mut items_guard = state.lock().unwrap();
    items_guard.push(new_item.clone());
    Json(new_item)
}
