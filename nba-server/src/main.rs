mod data;
mod routes;
use axum::{routing::get, Router};
use data::AppState;
use routes::{get_team, root};
use sqlx::{self, mysql::MySqlPoolOptions};

async fn get_pool() -> AppState {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:qwer1234@localhost/nba-data")
        .await
        .unwrap();
    AppState { pool }
}

#[tokio::main]
async fn main() {
    let state = get_pool().await;
    let app = Router::new()
        .route("/", get(root))
        .route("/team", get(get_team))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
