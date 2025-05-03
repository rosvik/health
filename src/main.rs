use config::Config;

mod config;
mod db;
mod poll;
mod serve;

#[tokio::main]
async fn main() {
    let pool = db::initialize_pool();
    let config = Config::load_from_file("config.toml");

    let serve_thread = tokio::spawn(serve::serve(pool.clone()));
    let poll_health_thread = tokio::spawn(poll::monitor(pool.clone(), config.clone()));

    serve_thread.await.unwrap();
    poll_health_thread.await.unwrap();
}
