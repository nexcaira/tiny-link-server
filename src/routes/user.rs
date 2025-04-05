use axum::{extract::Path, routing::{get, post}, Json, Router};
use serde_json::{json, Value};

async fn create_user() -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": "created user"
    }))
}

async fn get_user(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": format!("got user {}", id),
    }))
}

async fn edit_user(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": format!("edited user {}", id),
    }))
}

pub fn user_router() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user).patch(edit_user))
}