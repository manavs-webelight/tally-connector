use reqwest::Client;
use std::time::Duration;
use tracing::{error, info};

use crate::error::TallyClientError;

pub struct TallyClient {
    base_url: String,
    client: Client,
}

impl Clone for TallyClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            client: self.client.clone(),
        }
    }
}

impl TallyClient {
    pub fn new(base_url: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("reqwest client should build");
        Self { base_url, client }
    }

    pub async fn post_xml(&self, xml: &str, _request_name: &str) -> Result<String, TallyClientError> {
        let is_import = xml.contains("<IMPORTDATA");

        let text = match self.post_to_tally(&self.base_url, xml).await {
            Ok(resp) => resp,
            Err(e) if is_import => return Err(e),
            Err(_) => {
                let fallback_url = format!("{}?xmlrequest=true", self.base_url);
                info!(
                    "Primary response not valid XML, trying fallback: {}",
                    fallback_url
                );
                self.post_to_tally(&fallback_url, xml).await?
            }
        };

        Ok(text)
    }

    async fn post_to_tally(&self, url: &str, xml: &str) -> Result<String, TallyClientError> {
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/xml")
            .header("Accept", "application/xml")
            .body(xml.to_owned())
            .send()
            .await
            .map_err(|e| {
                error!("Tally request failed: {}", e);
                let err_msg = e.to_string();
                if err_msg.contains("Connection refused")
                    || err_msg.contains("connect timed out")
                    || err_msg.contains("tcp")
                {
                    TallyClientError("Your Tally server is disconnected, please start it.".to_string())
                } else {
                    TallyClientError(err_msg)
                }
            })?
            .error_for_status()
            .map_err(|e| TallyClientError(e.to_string()))?
            .text()
            .await
            .map_err(|e| TallyClientError(e.to_string()))?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tally_client_constructs() {
        let client = TallyClient::new("http://localhost:9000".to_string(), 30);
        assert_eq!(client.base_url, "http://localhost:9000");
    }
}
