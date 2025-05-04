use config::Config;

mod api;
mod config;
mod db;
mod poll;
mod serve;

#[tokio::main]
async fn main() {
    let config = Config::load_from_file("config.toml");
    let pool = db::get_pool(config.database.clone());
    db::try_setup_tables(&pool).unwrap();

    let serve_thread = tokio::spawn(serve::serve(pool.clone(), config.clone()));
    let poll_health_thread = tokio::spawn(poll::monitor(pool.clone(), config.clone()));

    serve_thread.await.unwrap();
    poll_health_thread.await.unwrap();
}
