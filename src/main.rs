use axum::{
    // http::StatusCode,
    extract::Path, response::Redirect, routing::get, Json, Router
};
use serde_json::{json, Value};
// use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/{id}", get(get_path))
        .fallback(fallback);

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({
        "code": 200,
        "msg": "Hello"
    }))
}

async fn fallback() -> Redirect {
    Redirect::to("/")
}

async fn get_path(Path(id): Path<String>) -> Json<Value> {
    let text = format!("current path is {}", id);
    
    Json(json!({
        "code": 200,
        "path": text,
    }))
}
