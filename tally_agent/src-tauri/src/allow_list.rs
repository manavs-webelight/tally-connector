use reqwest::Client;
use tracing::{error, warn};

#[derive(Clone)]
pub struct AllowListService {
    url: String,
    client: Client,
}

impl AllowListService {
    pub fn new(url: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("reqwest client should build");
        Self { url, client }
    }

    pub async fn is_request_allowed(&self, request_type: &str) -> bool {
        let url = format!("{}/allow?request_type={}", self.url, request_type);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                resp.json::<serde_json::Value>()
                    .await
                    .map(|v| v.get("allowed").and_then(|a| a.as_bool()).unwrap_or(false))
                    .unwrap_or(false)
            }
            Ok(_) => {
                warn!("Allow list returned non-200 for '{}'", request_type);
                false
            }
            Err(e) => {
                error!("Allow list check failed: {} — blocking request for safety", e);
                false
            }
        }
    }
}
