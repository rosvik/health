use crate::config::{Config, Endpoint, Notifier};
use crate::db::{self, Pool};
use tokio::time::{Duration, sleep};

pub async fn monitor(pool: Pool, config: Config) {
    loop {
        let mut endpoints_to_notify = Vec::new();
        println!("Checking for notifications");
        for endpoint in config.endpoints.clone() {
            let should_notify = should_notify(&pool, &endpoint).await;
            if should_notify {
                endpoints_to_notify.push(endpoint);
            }
        }
        println!("Endpoints to notify: {:?}", endpoints_to_notify);
        if !endpoints_to_notify.is_empty() {
            notify(&config, &endpoints_to_notify).await;
        }
        sleep(Duration::from_secs(60)).await;
    }
}

async fn should_notify(pool: &Pool, endpoint: &Endpoint) -> bool {
    let health_checks = db::get_health_checks(pool, &endpoint.name, 10)
        .await
        .unwrap();
    health_checks.iter().any(|hc| hc.status != 200)
}

async fn notify(config: &Config, endpoints: &[Endpoint]) {
    if let Some(notifiers) = &config.notify {
        for notifier in notifiers {
            match notifier {
                Notifier::Pushover => pushover_notify(endpoints).await,
            }
        }
    }
}

async fn pushover_notify(endpoints: &[Endpoint]) {
    let pushover_api_key = std::env::var("PUSHOVER_API_KEY").unwrap();
    let pushover_user_key = std::env::var("PUSHOVER_USER_KEY").unwrap();

    let message = format!(
        "{} is down",
        endpoints
            .iter()
            .map(|e| e.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.pushover.net/1/messages.json")
        .form(&[
            ("token", &pushover_api_key),
            ("user", &pushover_user_key),
            ("message", &message),
        ])
        .send()
        .await;

    if let Err(e) = &response {
        println!("Failed to send pushover notification: {:?}", e);
    }

    if let Ok(response) = response {
        let response_text = response.text().await.unwrap_or_default();
        println!("Sent pushover notification: {:?}", response_text);
    }
}
