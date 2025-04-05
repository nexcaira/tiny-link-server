use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

async fn create_path() -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": "created path",
    }))
}

async fn get_all_paths() -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": "got all paths",
        "paths": [
            ["my-github", "https://github.com/aenjojo"],
            ["my-twitter", "https://x.com/aenjojo_"],
            ["my-instagram", "https://www.instagram.com/aenjojo"],
        ],
    }))
}

async fn get_a_path(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": format!("got path {}", id),
    }))
}

async fn edit_path(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": format!("edited path {}", id),
    }))
}

async fn remove_path(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "code": 0,
        "message": format!("removed path {}", id),
    }))
}

pub fn path_router() -> Router {
    Router::new()
        .route("/path", post(create_path).get(get_a_path))
        .route(
            "/path/{id}",
            get(get_a_path).patch(edit_path).delete(remove_path),
        )
        .route("/paths", get(get_all_paths))
}
