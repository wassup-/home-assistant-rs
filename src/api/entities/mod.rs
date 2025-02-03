mod entity_id;
pub use entity_id::EntityId;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    Number(f64),
    String(String),
}

#[derive(Deserialize)]
pub struct ApiStatus {
    pub message: String,
}

impl ApiStatus {
    /// Returns `true` if the API is up and running.
    pub fn is_running(&self) -> bool {
        !self.message.is_empty()
    }
}

#[derive(Deserialize)]
pub struct Config {}

#[derive(Deserialize)]
pub struct Event {
    pub event: String,
    pub listener_count: u64,
}

pub type Events = Vec<Event>;

#[derive(Deserialize)]
pub struct Service {
    pub domain: String,
    pub services: Vec<String>,
}

pub type Services = Vec<Service>;

#[derive(Deserialize)]
pub struct State {
    pub entity_id: String,
    pub state: String,
    pub last_changed: String,
    pub attributes: HashMap<String, AttributeValue>,
}

pub type States = Vec<State>;

use serde::Deserialize;
use std::collections::HashMap;
