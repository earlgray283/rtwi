use std::collections::BTreeMap;
use anyhow::anyhow;
use reqwest::{Method, Response};
use super::Client;

impl Client {
    pub async fn show_profile(&self, name: Option<&str>, user_id: Option<&str>) -> Result<Response, anyhow::Error> {
        const ENDPOINT: &str = "https://api.twitter.com/1.1/users/show.json";

        if name.is_none() && user_id.is_none() {
            return Err(anyhow!("user_id or name must be set"));
        }

        let mut parms = BTreeMap::new();
        if let Some(name) = name {
            parms.insert("screen_name", name);
        } else if let Some(user_id) = user_id {
            parms.insert("user_id", user_id);
        }

        Ok(self.request(Method::GET, ENDPOINT, &parms).await?)
    }
}