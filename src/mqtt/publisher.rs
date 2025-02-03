pub struct Publisher {
    pub client: Client,
}

pub struct SpawnedPublisher;

impl Publisher {
    /// Spawns the runner.
    pub fn spawn(self, mut rx: Receiver<Message>, topic_fmt: TopicFormatter) -> SpawnedPublisher {
        let client = self.client;
        let _handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let topic = topic_fmt.sensor_state_topic(&msg.object_id);

                let bytes = msg.payload.to_bytes();
                client.publish(&topic, false, bytes).await.unwrap();
            }
        });
        SpawnedPublisher
    }
}

use super::{message::Message, topic::TopicFormatter, Client};
use tokio::sync::mpsc::Receiver;
