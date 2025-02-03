mod host;
mod supervisor;

/// The Supervisor client error type.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("REST client error: {0}")]
    RestClientError(#[from] RestClientError),
}

/// A Supervisor client.
pub struct Client {
    client: RestClient,
}

impl Client {
    /// Returns a new Supervisor client.
    pub fn new(base_url: &str, token: &str) -> Self {
        let client = RestClient::new(base_url, token);
        Client { client }
    }
}

use crate::rest_client::{RestClient, RestClientError};
