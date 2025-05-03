use crate::db::Pool;
use axum::{Router, extract::State, routing::get};
use std::sync::Arc;
use tokio::net::TcpListener;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn serve(pool: Pool) {
    let pool = Arc::new(pool);
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(|| async { "OK" }))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:8600").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(pool): State<Arc<Pool>>) -> String {
    let health_checks = crate::db::get_health_checks(&pool).await.unwrap();

    let mut response = format!("{CRATE_NAME} v{CRATE_VERSION}\n\n");

    for health_check in health_checks {
        response.push_str(&format!(
            "{}: {} in {}ms\n",
            health_check.name, health_check.status, health_check.response_time
        ));
    }

    response
}
