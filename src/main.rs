use axum::Router;
// use serde::{Deserialize, Serialize};
// use dotenvy;

mod routes;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();

    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not exist in ENV");

    let app = Router::new()
        .merge(routes::auth::auth_router())
        .merge(routes::path::path_router())
        .merge(routes::user::user_router());
        // .fallback(fallback);

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// async fn fallback() -> Redirect {
//     Redirect::to("/")
// }
