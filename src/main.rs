use config::Config;
use dotenv::dotenv;

mod api;
mod config;
mod db;
mod notify;
mod poll;
mod serve;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::load_from_file("config.toml");
    let pool = db::get_pool(config.database.clone());
    db::try_setup_tables(&pool).unwrap();

    let serve_thread = tokio::spawn(serve::serve(pool.clone(), config.clone()));
    let poll_health_thread = tokio::spawn(poll::monitor(pool.clone(), config.clone()));
    let notify_thread = tokio::spawn(notify::monitor(pool.clone(), config.clone()));

    serve_thread.await.unwrap();
    poll_health_thread.await.unwrap();
    notify_thread.await.unwrap();
}
