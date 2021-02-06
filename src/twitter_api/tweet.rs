use std::collections::BTreeMap;
use reqwest::{Method, Response};
use super::Client;

impl Client {
    pub async fn tweet(&self, status: &str) -> Result<Response, reqwest::Error> {
        const ENDPOINT: &str = "https://api.twitter.com/1.1/statuses/update.json";

        let mut params = BTreeMap::new();
        params.insert("status", status);

        self.request(Method::POST, ENDPOINT, &params).await
    }
}