/// The client error type.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("REST client error: {0}")]
    RestClientError(#[from] RestClientError),
}

/// An API client.
pub struct Client {
    client: RestClient,
}

impl Client {
    pub fn new(base_url: &str, token: &str) -> Self {
        let client = RestClient::new(base_url, token);
        Client { client }
    }

    /// Returns `true` if the API is up and running, `false` otherwise.
    pub async fn get_status(&self) -> Result<entities::ApiStatus, ClientError> {
        let resp = self.client.get("", ()).await?;
        Ok(resp)
    }

    /// Returns the current configuration.
    pub async fn get_config(&self) -> Result<entities::Config, ClientError> {
        let resp = self.client.get("config", ()).await?;
        Ok(resp)
    }

    /// Returns an array of event objects.
    pub async fn get_events(&self) -> Result<entities::Events, ClientError> {
        let resp = self.client.get("events", ()).await?;
        Ok(resp)
    }

    /// Returns an array of service objects.
    pub async fn get_services(&self) -> Result<entities::Services, ClientError> {
        let resp = self.client.get("services", ()).await?;
        Ok(resp)
    }

    /// Returns an array of state objects.
    pub async fn get_states(&self) -> Result<entities::States, ClientError> {
        let resp = self.client.get("states", ()).await?;
        Ok(resp)
    }

    /// Returns an array of state objects.
    pub async fn get_states_for_entity(
        &self,
        entity_id: &str,
    ) -> Result<entities::States, ClientError> {
        let resp = self.client.get(&format!("states/{entity_id}"), ()).await?;
        Ok(resp)
    }

    pub async fn call_service<T>(
        &self,
        domain: &str,
        service: &str,
        data: T,
    ) -> Result<entities::States, ClientError>
    where
        T: Serialize,
    {
        let resp = self
            .client
            .post(&format!("services/{domain}/{service}"), data)
            .await?;
        Ok(resp)
    }
}

use crate::{
    api::entities,
    rest_client::{RestClient, RestClientError},
};
use serde::Serialize;
