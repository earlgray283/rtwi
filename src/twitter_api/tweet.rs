use super::Client;
use anyhow::anyhow;
use reqwest::{Method, StatusCode};
use std::collections::BTreeMap;

impl Client {
    pub async fn tweet(&self, status: &str) -> Result<(), anyhow::Error> {
        const ENDPOINT: &str = "https://api.twitter.com/1.1/statuses/update.json";

        let mut params = BTreeMap::new();
        params.insert("status", status);

        let res = self.request(Method::POST, ENDPOINT, &params).await?;
        if res.status() != StatusCode::OK {
            Err(anyhow!(format!("status-code: {}", res.status())))
        } else {
            Ok(())
        }
    }
}
