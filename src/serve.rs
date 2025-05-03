use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;
use tokio::net::TcpListener;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn serve(pool: r2d2::Pool<SqliteConnectionManager>) {
    let pool = Arc::new(pool);
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(|| async { "OK" }))
        .route("/add", post(add_endpoint).with_state(pool));

    let listener = TcpListener::bind("127.0.0.1:8600").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    format!("{CRATE_NAME} v{CRATE_VERSION}")
}

async fn add_endpoint(
    State(pool): State<Arc<r2d2::Pool<SqliteConnectionManager>>>,
    Json(endpoint): Json<crate::db::EndpointPayload>,
) -> impl IntoResponse {
    match crate::db::add_endpoint(&pool, endpoint).await {
        Ok(_) => (StatusCode::CREATED, "Endpoint added"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error adding endpoint"),
    }
}
