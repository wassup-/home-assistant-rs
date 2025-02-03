/// The REST client error type.
#[derive(thiserror::Error, Debug)]
#[error("REST client error: {0}")]
pub struct RestClientError(#[from] reqwest::Error);

/// A REST client.
pub struct RestClient {
    client: reqwest::Client,
    base_url: String,
}

impl RestClient {
    /// Returns a new REST client.
    ///
    /// # Panics
    ///
    /// This function panics if the token could not be parsed.
    pub fn new(base_url: &str, token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {token}").parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        RestClient {
            client,
            base_url: base_url.to_owned(),
        }
    }

    pub async fn get<T, Q>(&self, path: &str, query: Q) -> Result<T, RestClientError>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let resp = self
            .client
            .get(self.build_url(path))
            .query(&query)
            .send()
            .await?;
        let res = resp.json().await?;
        Ok(res)
    }

    pub async fn post<T, B>(&self, path: &str, json: B) -> Result<T, RestClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let resp = self
            .client
            .post(self.build_url(path))
            .json(&json)
            .send()
            .await?;
        let res = resp.json().await?;
        Ok(res)
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{path}", self.base_url)
    }
}

use reqwest::header::{self, HeaderMap};
use serde::{de::DeserializeOwned, Serialize};
