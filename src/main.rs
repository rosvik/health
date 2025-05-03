mod serve;

#[tokio::main]
async fn main() {
    // Make a separate thread for the serve function
    let serve_thread = tokio::spawn(serve::serve());
    // Make a separate thread for the poll_health function
    let poll_health_thread = tokio::spawn(poll_health());

    // Wait for the serve thread to finish
    serve_thread.await.unwrap();
    // Wait for the poll_health thread to finish
    poll_health_thread.await.unwrap();
}

async fn poll_health() {
    loop {
        let response = reqwest::get("http://localhost:8600/health").await.unwrap();
        println!("{}", response.text().await.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
