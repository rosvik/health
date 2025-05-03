use crate::config::{Config, Endpoint};
use crate::db::{self, Pool};
use tokio::time::{Duration, Instant, sleep};

pub async fn monitor(pool: Pool, config: Config) {
    loop {
        for endpoint in config.endpoints.clone() {
            poll(pool.clone(), endpoint).await;
        }
        sleep(Duration::from_millis(config.interval)).await;
    }
}

pub async fn poll(pool: Pool, endpoint: Endpoint) {
    let start = Instant::now();
    let response = reqwest::get(endpoint.url).await.unwrap();
    let response_time = start.elapsed().as_millis();

    let status = response.status();

    db::insert_health_check(
        &pool,
        db::HealthCheckRow {
            name: endpoint.name.clone(),
            status: status.into(),
            response_time: response_time as u64,
        },
    )
    .await
    .unwrap();

    if !status.is_success() {
        println!("Error: {} - {}", endpoint.name, status);
    }
}
