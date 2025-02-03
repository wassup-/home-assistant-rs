pub struct Subscriber {
    pub state: SharedState,
}

#[derive(Clone)]
pub struct SpawnedSubscriber;

impl Subscriber {
    /// Spawns the poller.
    pub fn spawn(self, mut eventloop: EventLoop) -> SpawnedSubscriber {
        let state = self.state;
        let _handle = tokio::spawn(async move {
            loop {
                let publish = match eventloop.poll().await {
                    Ok(Event::Incoming(Incoming::Publish(publish))) => publish,
                    Ok(event) => {
                        log::trace!("ignoring event {:?}", event);
                        continue;
                    }
                    Err(err) => {
                        log::error!("got error {:?}", err);
                        continue;
                    }
                };

                log::trace!("got publish {:?}", publish);
                handle_event(publish, &state).await;
            }
        });
        SpawnedSubscriber
    }
}

async fn handle_event(publish: Publish, state: &SharedState) {
    let (topic, payload) = (publish.topic, publish.payload);
    let Some(topic) = topic_defmt(&topic) else {
        log::trace!("ignoring publish for invalid topic: {}", topic);
        return;
    };

    match topic.component.as_str() {
        "button" => handle_button_event(topic, &payload, state).await,
        "number" => handle_number_event(topic, &payload, state).await,
        "switch" => handle_switch_event(topic, &payload, state).await,
        _ => {
            log::trace!(
                "ignoring publish for unknown component: {}",
                topic.component
            );
        }
    }
}

async fn handle_button_event(topic: Topic, _payload: &[u8], state: &SharedState) {
    if topic.suffix != "cmd" {
        log::trace!("ignoring publish unknown suffix: {}", topic.suffix);
        return;
    }

    let state = state.lock().await;
    let Some(button_state) = state.buttons.get(&topic.object_id) else {
        log::trace!("ignoring publish for unknown button: {}", topic.object_id);
        return;
    };
    button_state.press().await;
}

async fn handle_switch_event(topic: Topic, payload: &[u8], state: &SharedState) {
    if topic.suffix != "cmd" {
        log::trace!("ignoring publish unknown suffix: {}", topic.suffix);
        return;
    }

    let state = state.lock().await;
    let Some(switch_state) = state.switches.get(&topic.object_id) else {
        log::trace!("ignoring publish for unknown switch: {}", topic.object_id);
        return;
    };

    let Some(value) = String::from_utf8(payload.to_vec()).ok() else {
        return;
    };
    switch_state.update_value(value).await;
}

async fn handle_number_event(topic: Topic, payload: &[u8], state: &SharedState) {
    if topic.suffix != "cmd" {
        log::trace!("ignoring publish unknown suffix: {}", topic.suffix);
        return;
    }

    let state = state.lock().await;
    let Some(number) = state.numbers.get(&topic.object_id) else {
        log::trace!("ignoring publish for unknown number: {}", topic.object_id);
        return;
    };

    let Some(value) = String::from_utf8(payload.to_vec())
        .ok()
        .and_then(|s| s.parse().ok())
    else {
        return;
    };

    println!(
        "updating value of number {} to {}",
        number.object_id(),
        value
    );
    number.update_value(value).await;
}

use super::{
    state::SharedState,
    topic::{topic_defmt, Topic},
};
use rumqttc::{Event, EventLoop, Incoming, Publish};
