use axum::{routing::post, routing::delete, Json, Router};
use serde_json::{json, Value};

async fn sign_in() -> Json<Value> {
    Json(json!({
        "active": 1
    }))
}

async fn sign_out() -> Json<Value> {
    Json(json!({
        "active": 0
    }))
}

pub fn auth_router() -> Router {
    Router::new()
        .route("/auth/signin", post(sign_in))
        .route("/auth/session", delete(sign_out))
}