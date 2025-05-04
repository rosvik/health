use std::sync::Arc;

use crate::serve::ServerState;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::db;
pub fn api_router(state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/checks/{name}", get(list_checks))
        .with_state(state)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListResponse {
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

    let response = ListResponse {
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
