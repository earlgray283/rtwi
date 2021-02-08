use super::Client;
use anyhow::anyhow;
use reqwest::{Method, StatusCode};
use serde_json::Value;
use std::collections::BTreeMap;

pub struct UserInfo {
    pub name: String,
    pub screen_name: String,
    pub bio: String,
}
impl Client {
    pub async fn get_profile(
        &self,
        name: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<UserInfo, anyhow::Error> {
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

        let res = self.request(Method::GET, ENDPOINT, &parms).await?;
        if res.status() != StatusCode::OK {
            return Err(anyhow!("failed to get status."));
        }

        let value = serde_json::from_str::<Value>(&res.text().await?)?;

        Ok(UserInfo {
            name: value["name"].to_string(),
            screen_name: value["screen_name"].to_string(),
            bio: value["description"].to_string(),
        })
    }
}
