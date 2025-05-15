use crate::{api, config::Config, db::Pool};
use axum::{Router, routing::get_service};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct ServerState {
    pub pool: Pool,
    pub config: Config,
}

pub async fn serve(pool: Pool, config: Config) {
    let state = Arc::new(ServerState { pool, config });
    let app = Router::new()
        .nest_service("/api/health/v1", api::api_router(state.clone()))
        .fallback_service(get_service(ServeDir::new("dist")))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8603").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
