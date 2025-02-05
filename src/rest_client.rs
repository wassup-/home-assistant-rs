/// The REST client error type.
#[derive(thiserror::Error, Debug)]
#[error("REST client error: {0}")]
pub struct RestClientError(String);

/// A REST client.
pub struct RestClient {
    client: reqwest::Client,
    base_url: Url,
}

impl RestClient {
    /// Returns a new REST client.
    ///
    /// # Panics
    ///
    /// This function panics if either the base url or the token could not be parsed.
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
        let base_url = ::url::Url::parse(base_url).unwrap();
        RestClient { client, base_url }
    }

    pub async fn get<T, Q>(&self, path: &str, query: Q) -> Result<T, RestClientError>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.base_url.join(path)?;
        let resp = self.client.get(url).query(&query).send().await?;
        let res = resp.json().await?;
        Ok(res)
    }

    pub async fn post<T, B>(&self, path: &str, json: B) -> Result<T, RestClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.base_url.join(path)?;
        let resp = self.client.post(url).json(&json).send().await?;
        let res = resp.json().await?;
        Ok(res)
    }
}

impl From<url::ParseError> for RestClientError {
    fn from(err: url::ParseError) -> Self {
        RestClientError(err.to_string())
    }
}

impl From<reqwest::Error> for RestClientError {
    fn from(err: reqwest::Error) -> Self {
        RestClientError(err.to_string())
    }
}

use reqwest::header::{self, HeaderMap};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;
