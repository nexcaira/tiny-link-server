use axum::{
    // http::StatusCode,
    // extract::Path, response::Redirect, routing::get, Json, 
    Router
};
// use serde_json::{json, Value};
// use serde::{Deserialize, Serialize};
// use dotenvy;

pub mod routes;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();

    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not exist in ENV");

    let app = Router::new()
        // .route("/", get(root))
        .merge(routes::app::auth::auth_router())
        .merge(routes::app::path::path_router())
        .merge(routes::app::user::user_router());
        // .fallback(fallback);

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// async fn root() -> Json<Value> {
//     Json(json!({
//         "code": 200,
//         "msg": "Hello"
//     }))
// }

// async fn fallback() -> Redirect {
//     Redirect::to("/")
// }
