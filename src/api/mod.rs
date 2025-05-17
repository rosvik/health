use crate::db;
use crate::{config::Endpoint, serve::ServerState};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn api_router(state: Arc<ServerState>) -> Router {
    let cors = cors::CorsLayer::new().allow_origin(cors::Any);
    Router::new()
        .route(
            "/",
            get(|| async { format!("{CRATE_NAME} v{CRATE_VERSION}") }),
        )
        .route("/endpoints", get(list_endpoints))
        .route("/checks/{name}", get(list_checks))
        .with_state(state)
        .layer(cors)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListEndpointsResponse {
    endpoints: Vec<Endpoint>,
}
async fn list_endpoints(State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    let endpoints = state.config.endpoints.clone();
    (StatusCode::OK, Json(ListEndpointsResponse { endpoints })).into_response()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListChecksResponse {
    name: String,
    url: String,
    interval: u64,
    checks: Vec<Check>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Check {
    status: u16,
    response_time: u64,
    created_at: String,
}

#[derive(Deserialize)]
struct ListParams {
    #[serde(default = "default_limit")]
    limit: u32,
}
fn default_limit() -> u32 {
    100
}
async fn list_checks(
    State(state): State<Arc<ServerState>>,
    Path(name): Path<String>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let health_checks = match db::get_health_checks(&state.pool, &name, params.limit).await {
        Ok(health_checks) => health_checks,
        Err(_) => return (StatusCode::NOT_FOUND, "Endpoint not found").into_response(),
    };
    let config = match crate::config::endpoint_config(&state.config, &name) {
        Some(config) => config,
        None => return (StatusCode::NOT_FOUND, "Endpoint not found").into_response(),
    };

    let response = ListChecksResponse {
        name: config.name,
        url: config.url,
        interval: state.config.interval,
        checks: health_checks
            .into_iter()
            .map(|h| Check {
                status: h.status,
                response_time: h.response_time,
                created_at: h.created_at.unwrap_or_default(),
            })
            .collect(),
    };

    (StatusCode::OK, Json(response)).into_response()
}
