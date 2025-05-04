use crate::{
    api,
    config::Config,
    db::{HealthCheckRow, Pool},
};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::net::TcpListener;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct ServerState {
    pub pool: Pool,
    pub config: Config,
}

pub async fn serve(pool: Pool, config: Config) {
    let state = Arc::new(ServerState { pool, config });
    let app = Router::new()
        .route("/", get(handler))
        .nest_service("/api/health/v1", api::api_router(state.clone()))
        .route("/{name}", get(handler_with_name))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8603").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<ServerState>>) -> String {
    let mut response = format!("{CRATE_NAME} v{CRATE_VERSION}\n");

    let config = state.config.clone();
    response.push_str(&format!("Interval: {}s\n\n", config.interval));

    for endpoint in state.config.endpoints.clone() {
        let health_checks = crate::db::get_health_checks(&state.pool, &endpoint.name, 100)
            .await
            .unwrap();
        response.push_str(&format!(
            "{:<15}: ",
            endpoint.name.clone().chars().take(15).collect::<String>()
        ));

        response.push_str(&get_timeline_status(&health_checks));
        response.push('\n');
    }
    response
}

async fn handler_with_name(
    State(state): State<Arc<ServerState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let health_checks = match crate::db::get_health_checks(&state.pool, &name, 1000).await {
        Ok(health_checks) => health_checks,
        Err(_) => return format!("Endpoint {} not found", name).into_response(),
    };
    let config = match crate::config::endpoint_config(&state.config, &name) {
        Some(config) => config,
        None => return (StatusCode::NOT_FOUND, "Endpoint not found").into_response(),
    };

    let mut response = format!("{}\n{}\n\n", config.name, config.url);

    let mut hourly_checks = Vec::new();
    let mut current_hour = String::new();
    for health_check in health_checks.clone() {
        let created_at = health_check.created_at.clone().unwrap();
        let hour = created_at.chars().take(13).collect::<String>(); // "2025-05-03 19"
        if hour == current_hour {
            hourly_checks.push(health_check);
        } else {
            response.push_str(&format!(
                "{}: {}\n",
                current_hour,
                get_timeline_status(&hourly_checks)
            ));
            current_hour = hour;
            hourly_checks = Vec::from([health_check]);
        }
    }

    response.push_str("\n\n");

    for health_check in health_checks {
        response.push_str(&format!(
            "{} {:?} {:?}\n",
            health_check.created_at.unwrap(),
            health_check.status,
            health_check.response_time,
        ));
    }

    response.into_response()
}

fn get_timeline_status(health_checks: &Vec<HealthCheckRow>) -> String {
    let mut response = String::new();
    for health_check in health_checks {
        if health_check.status != 200 {
            response.push('X');
        } else if health_check.response_time > 1000 {
            response.push('‾');
        } else if health_check.response_time > 500 {
            response.push('-');
        } else {
            response.push('_');
        }
    }
    response
}
