use super::config::Config;
use anyhow::Result;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod tweet;
pub mod user;

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub api_key: String,
    pub api_secret_key: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            api_key: config.twitter_api_info.api_key.clone(),
            api_secret_key: config.twitter_api_info.api_secret_key.clone(),
            access_token: config.twitter_api_info.access_token.clone(),
            access_token_secret: config.twitter_api_info.access_token_secret.clone(),
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        url: &str,
        params: &BTreeMap<&str, &str>,
    ) -> Result<Response, reqwest::Error> {
        let header_map = self.header(&method, url, params).await;
        let url_with_params = format!(
            "{}?{}",
            url,
            params
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(&str, &str)>>()
                .equal_collect()
                .join("&")
        );

        let client = reqwest::Client::new();
        let request = client.request(method, &url_with_params).headers(header_map);
        request.send().await
    }

    async fn header(
        &self,
        method: &reqwest::Method,
        url: &str,
        params: &BTreeMap<&str, &str>,
    ) -> HeaderMap {
        let mut map = HeaderMap::new();

        map.insert(
            AUTHORIZATION,
            self.auth(method, url, params).await.parse().unwrap(),
        );
        map.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        map
    }

    async fn auth(
        &self,
        method: &reqwest::Method,
        url: &str,
        params: &BTreeMap<&str, &str>,
    ) -> String {
        let timestamp = format!("{}", chrono::Utc::now().timestamp());

        let nonce = {
            let mut rng = thread_rng();
            std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(32)
                .collect::<String>()
        };

        let mut others: Vec<(&str, &str)> = vec![
            ("oauth_consumer_key", &self.api_key),
            ("oauth_token", &self.access_token),
            ("oauth_signature_method", "HMAC-SHA1"),
            ("oauth_version", "1.0"),
            ("oauth_timestamp", &timestamp),
            ("oauth_nonce", &nonce),
        ];

        let signature = self.signature(method, url, params.clone(), &others).await;

        others.push(("oauth_signature", &signature));

        format!("OAuth {}", others.equal_collect().join(", "))
    }

    async fn signature(
        &self,
        method: &reqwest::Method,
        url: &str,
        mut params: BTreeMap<&str, &str>,
        others: &Vec<(&str, &str)>,
    ) -> String {
        for (k, v) in others {
            params.insert(k, v);
        }

        let param_str = params
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(&str, &str)>>()
            .equal_collect()
            .join("&");

        let signature_base_string = format!(
            "{}&{}&{}",
            method,
            percent_encode(url),
            percent_encode(&param_str)
        );

        let signing_key = format!("{}&{}", self.api_secret_key, self.access_token_secret);
        base64::encode(hmacsha1::hmac_sha1(
            signing_key.as_bytes(),
            signature_base_string.as_bytes(),
        ))
    }
}

fn percent_encode(s: &str) -> percent_encoding::PercentEncode {
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'*')
        .remove(b'-')
        .remove(b'.')
        .remove(b'_');

    utf8_percent_encode(s, FRAGMENT)
}

trait Collect {
    fn equal_collect(&self) -> Vec<String>;
}

impl Collect for Vec<(&str, &str)> {
    fn equal_collect(&self) -> Vec<String> {
        self.iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect()
    }
}
