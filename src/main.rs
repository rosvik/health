mod db;
mod poll;
mod serve;

#[tokio::main]
async fn main() {
    let pool = db::initialize_pool();

    let serve_thread = tokio::spawn(serve::serve(pool.clone()));
    let poll_health_thread = tokio::spawn(poll::poll_health(pool.clone()));

    serve_thread.await.unwrap();
    poll_health_thread.await.unwrap();
}
