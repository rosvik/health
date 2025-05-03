use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;

pub fn initialize_pool() -> r2d2::Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::memory();
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(include_str!("../../sql/setup.sql"))
        .unwrap();
    pool
}

#[derive(Deserialize)]
pub struct EndpointPayload {
    pub url: String,
    pub interval: u64,
}

pub async fn add_endpoint(
    pool: &r2d2::Pool<SqliteConnectionManager>,
    endpoint: EndpointPayload,
) -> Result<(), String> {
    let conn = pool.get().unwrap();
    match conn.execute(
        "INSERT INTO endpoints (url, interval) VALUES (?, ?)",
        [&endpoint.url, &endpoint.interval.to_string()],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error adding endpoint: {}", e);
            Err(e.to_string())
        }
    }
}

#[derive(Deserialize)]
pub struct HealthCheckPayload {
    pub endpoint_id: u64,
    pub status: u16,
    pub response_time: u64,
}
pub async fn insert_health_check(
    pool: &r2d2::Pool<SqliteConnectionManager>,
    endpoint: HealthCheckPayload,
) -> Result<(), String> {
    let conn = pool.get().unwrap();
    match conn.execute(
        "INSERT INTO observation (endpoint_id, status, response_time) VALUES (?, ?, ?)",
        [
            &endpoint.endpoint_id,
            &u64::from(endpoint.status),
            &endpoint.response_time,
        ],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error inserting health check: {}", e);
            Err(e.to_string())
        }
    }
}

#[tokio::test]
async fn test_add_endpoint() {
    let pool = initialize_pool();
    let endpoint = EndpointPayload {
        url: "https://example.com".to_string(),
        interval: 1000,
    };
    assert!(add_endpoint(&pool, endpoint).await.is_ok());
}

#[tokio::test]
async fn test_insert_health_check() {
    let pool = initialize_pool();
    add_endpoint(
        &pool,
        EndpointPayload {
            url: "https://example.com".to_string(),
            interval: 1000,
        },
    )
    .await
    .unwrap();
    let health_check = HealthCheckPayload {
        endpoint_id: 0,
        status: 200,
        response_time: 100,
    };
    assert!(insert_health_check(&pool, health_check).await.is_ok());
}
