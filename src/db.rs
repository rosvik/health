use serde::Deserialize;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub fn initialize_pool() -> Pool {
    let manager = r2d2_sqlite::SqliteConnectionManager::memory();
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(include_str!("../sql/setup.sql"))
        .unwrap();
    pool
}

#[derive(Deserialize)]
pub struct HealthCheckPayload {
    pub name: String,
    pub status: u16,
    pub response_time: u64,
}
pub async fn insert_health_check(pool: &Pool, endpoint: HealthCheckPayload) -> Result<(), String> {
    let conn = pool.get().unwrap();
    match conn.execute(
        "INSERT INTO health_checks (name, status, response_time) VALUES (?, ?, ?)",
        [
            &endpoint.name,
            &endpoint.status.to_string(),
            &endpoint.response_time.to_string(),
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
async fn test_insert_health_check() {
    let pool = initialize_pool();
    let health_check = HealthCheckPayload {
        name: "example.com".to_string(),
        status: 200,
        response_time: 100,
    };
    assert!(insert_health_check(&pool, health_check).await.is_ok());
}
