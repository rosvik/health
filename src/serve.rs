use axum::{Router, routing::get};
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;
use tokio::net::TcpListener;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn serve(pool: r2d2::Pool<SqliteConnectionManager>) {
    let _pool = Arc::new(pool);
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(|| async { "OK" }));

    let listener = TcpListener::bind("127.0.0.1:8600").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    format!("{CRATE_NAME} v{CRATE_VERSION}")
}
