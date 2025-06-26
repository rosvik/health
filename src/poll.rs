use crate::config::{Config, Endpoint};
use crate::db::{self, Pool};
use tokio::time::{Duration, Instant, sleep};

const REPORT_INTERVAL_SECS: u64 = 60 * 10;

pub async fn monitor(pool: Pool, config: Config) {
    let mut failed_rows: Vec<db::HealthCheckRow> = Vec::new();
    let mut last_report = Instant::now();
    loop {
        for endpoint in config.endpoints.clone() {
            let row = poll(pool.clone(), endpoint).await;
            if row.status != 200 {
                failed_rows.push(row);
            }
        }
        sleep(Duration::from_secs(config.interval)).await;

        // Report errors every REPORT_INTERVAL_SECS
        if last_report.elapsed() > Duration::from_secs(REPORT_INTERVAL_SECS) {
            report_errors_to_pushover(failed_rows.clone()).await;
            failed_rows.clear();
            last_report = Instant::now();
        }
    }
}

pub async fn poll(pool: Pool, endpoint: Endpoint) -> db::HealthCheckRow {
    let start = Instant::now();
    let response = match reqwest::get(endpoint.url).await {
        Ok(response) => response,
        Err(e) => {
            println!("Failed to poll {}: {}", endpoint.name, e);
            let response_time = start.elapsed().as_millis();
            let row = db::HealthCheckRow {
                name: endpoint.name.clone(),
                status: 0,
                response_time: response_time as u64,
                created_at: None,
            };
            db::insert_health_check(&pool, &row).await.unwrap();
            return row;
        }
    };
    let response_time = start.elapsed().as_millis();

    let status = response.status();

    let row = db::HealthCheckRow {
        name: endpoint.name.clone(),
        status: status.into(),
        response_time: response_time as u64,
        created_at: None,
    };
    db::insert_health_check(&pool, &row).await.unwrap();

    if !status.is_success() {
        println!("Error: {} - {}", endpoint.name, status);
    }
    row
}

/// https://pushover.net/api#messages
async fn report_errors_to_pushover(failed_rows: Vec<db::HealthCheckRow>) {
    let pushover_token = std::env::var("PUSHOVER_TOKEN").unwrap();
    let pushover_user = std::env::var("PUSHOVER_USER_KEY").unwrap();
    let client = reqwest::Client::new();
    let message = failed_rows
        .iter()
        .map(|row| format!("{} - {}", row.name, row.status))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Reporting errors: {}", message);

    let body = format!(
        "token={}&user={}&message={message}",
        pushover_token, pushover_user
    );
    let _ = client
        .post("https://api.pushover.net/1/messages.json")
        .body(body)
        .send()
        .await;
}
