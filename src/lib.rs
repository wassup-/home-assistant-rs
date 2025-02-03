#![deny(unsafe_code)]

/// The API module.
#[cfg(feature = "api")]
pub mod api;

/// The MQTT module.
#[cfg(feature = "mqtt")]
pub mod mqtt;

mod rest_client;

/// The Supervisor module.
#[cfg(feature = "supervisor")]
pub mod supervisor;

/// The global error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// API error.
    #[error("API error: {0}")]
    Api(#[from] api::ClientError),
    /// Supervisor error.
    #[error("Supervisor error: {0}")]
    Supervisor(#[from] supervisor::ClientError),
}
