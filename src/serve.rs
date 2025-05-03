use crate::{
    config::Config,
    db::{HealthCheckRow, Pool},
};
use axum::{
    Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::net::TcpListener;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

struct ServerState {
    pool: Pool,
    config: Config,
}

pub async fn serve(pool: Pool, config: Config) {
    let state = Arc::new(ServerState { pool, config });
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(|| async { "OK" }))
        .route("/{name}", get(handler_with_name))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8600").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<ServerState>>) -> String {
    let mut response = format!("{CRATE_NAME} v{CRATE_VERSION}\n");

    let config = state.config.clone();
    response.push_str(&format!("Interval: {}s\n\n", config.interval));

    for endpoint in state.config.endpoints.clone() {
        let health_checks = crate::db::get_health_checks(&state.pool, endpoint.name.clone(), 100)
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
    let health_checks = match crate::db::get_health_checks(&state.pool, name.clone(), 1000).await {
        Ok(health_checks) => health_checks,
        Err(_) => {
            return format!("Endpoint {} not found", name).into_response();
        }
    };

    let config = state
        .config
        .endpoints
        .iter()
        .find(|e| e.name == name)
        .unwrap();

    let mut response = format!("{}\n{}\n\n", config.name, config.url);

    response.push_str(&get_timeline_status(&health_checks));
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
