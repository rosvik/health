use crate::db;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::time::{Duration, Instant, sleep};

pub async fn poll_health(pool: r2d2::Pool<SqliteConnectionManager>) {
    loop {
        let start = Instant::now();
        let response = reqwest::get("http://localhost:8600/health").await.unwrap();

        let status = response.status();
        let response_time = start.elapsed().as_millis();
        db::insert_health_check(
            &pool,
            db::HealthCheckPayload {
                name: "health".to_string(),
                status: status.into(),
                response_time: response_time as u64,
            },
        )
        .await
        .unwrap();
        println!("{}", response.text().await.unwrap());
        sleep(Duration::from_secs(1)).await;
    }
}
