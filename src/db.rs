use serde::Deserialize;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub fn get_pool(database_location: Option<String>) -> Pool {
    let manager = match database_location {
        Some(location) => r2d2_sqlite::SqliteConnectionManager::file(location),
        None => r2d2_sqlite::SqliteConnectionManager::memory(),
    };
    r2d2::Pool::new(manager).unwrap()
}

pub fn try_setup_tables(pool: &Pool) -> Result<(), String> {
    let conn = pool.get().unwrap();
    let mut result = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='health_checks'")
        .map_err(|e| e.to_string())?;
    let row = result.query_row([], |row| row.get::<usize, String>(0));
    if row.is_err() {
        conn.execute_batch(include_str!("../sql/setup.sql"))
            .map_err(|e| e.to_string())?
    }
    Ok(())
}

#[derive(Deserialize, Clone)]
pub struct HealthCheckRow {
    pub name: String,
    pub status: u16,
    pub response_time: u64,
    pub created_at: Option<String>,
}
pub async fn insert_health_check(pool: &Pool, endpoint: HealthCheckRow) -> Result<(), String> {
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
pub async fn get_health_checks(
    pool: &Pool,
    name: &String,
    limit: u32,
) -> Result<Vec<HealthCheckRow>, String> {
    let conn = pool.get().unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT name, status, response_time, created_at FROM health_checks WHERE name = ? ORDER BY created_at DESC LIMIT ?",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([name, &limit.to_string()], |row| {
            Ok(HealthCheckRow {
                name: row.get(0)?,
                status: row.get(1)?,
                response_time: row.get(2)?,
                created_at: Some(row.get(3)?),
            })
        })
        .map_err(|e| e.to_string())?;

    let mut health_checks = Vec::new();
    for row in rows {
        health_checks.push(row.map_err(|e| e.to_string())?);
    }

    Ok(health_checks)
}

#[tokio::test]
async fn test_health_check() {
    let pool = get_pool(None);
    try_setup_tables(&pool).unwrap();
    let health_check = HealthCheckRow {
        name: "example.com".to_string(),
        status: 200,
        response_time: 100,
        created_at: None,
    };
    assert!(insert_health_check(&pool, health_check).await.is_ok());

    let health_checks = get_health_checks(&pool, &String::from("example.com"), 100)
        .await
        .unwrap();
    assert_eq!(health_checks.len(), 1);
    assert_eq!(health_checks[0].name, "example.com");
    assert_eq!(health_checks[0].status, 200);
    assert_eq!(health_checks[0].response_time, 100);
}
