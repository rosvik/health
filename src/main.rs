use r2d2_sqlite::SqliteConnectionManager;
use tokio::time::{Duration, Instant, sleep};

mod db;
mod serve;

#[tokio::main]
async fn main() {
    let pool = db::initialize_pool();

    let serve_thread = tokio::spawn(serve::serve(pool.clone()));
    let poll_health_thread = tokio::spawn(poll_health(pool.clone()));

    serve_thread.await.unwrap();
    poll_health_thread.await.unwrap();
}

async fn poll_health(pool: r2d2::Pool<SqliteConnectionManager>) {
    loop {
        let start = Instant::now();
        let response = reqwest::get("http://localhost:8600/health").await.unwrap();

        let status = response.status();
        let response_time = start.elapsed().as_millis();
        db::insert_health_check(
            &pool,
            db::HealthCheckPayload {
                endpoint_id: 1,
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
