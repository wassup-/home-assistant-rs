#[derive(Clone)]
pub struct State<Value> {
    object_id: ObjectId,
    value: Arc<Mutex<Value>>,
    tx: Sender<Message>,
}

impl<Value> State<Value> {
    pub fn new(object_id: ObjectId, value: Value, tx: Sender<Message>) -> Self {
        State {
            object_id,
            value: Arc::new(Mutex::new(value)),
            tx,
        }
    }

    /// Returns the object id.
    pub fn object_id(&self) -> &ObjectId {
        &self.object_id
    }
}

impl<Value: Clone> State<Value> {
    /// Updates and publishes the value.
    pub async fn update_value(&self, new_value: Value) {
        let mut value = self.value.lock().await;
        *value = new_value;
        self.publish_value().await
    }

    /// Published the value.
    pub async fn publish_value(&self) {
        todo!()
    }
}

use crate::mqtt::{message::Message, ObjectId};
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};
