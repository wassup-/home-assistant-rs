/// The client error type.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ClientError(#[from] rumqttc::ClientError);

const TX_CAPACITY: usize = 32;

/// MQTT client credentials.
pub struct ClientCredentials {
    pub username: String,
    pub password: String,
}

/// MQTT client configuration.
pub struct ClientConfig {
    pub client_id: String,
    pub credentials: Option<ClientCredentials>,
    pub hass_domain: String,
    pub addon_domain: String,
}

/// An MQTT client.
#[derive(Clone)]
pub struct Client {
    client: AsyncClient,
    state: SharedState,
    tx: Sender<Message>,
    topic_fmt: TopicFormatter,
}

impl Client {
    pub fn new(host: &str, port: u16, config: ClientConfig) -> Self {
        let state = Arc::new(Mutex::new(State::default()));

        let mut client_options = MqttOptions::new(config.client_id, host, port);
        if let Some(credentials) = config.credentials {
            client_options.set_credentials(credentials.username, credentials.password);
        }

        let (client, eventloop) = AsyncClient::new(client_options, TX_CAPACITY);
        let (tx, rx) = channel(TX_CAPACITY);

        let topic_fmt = TopicFormatter::new(config.hass_domain, config.addon_domain);

        Subscriber {
            state: Arc::clone(&state),
        }
        .spawn(eventloop);

        let client = Client {
            client,
            state,
            tx,
            topic_fmt: topic_fmt.clone(),
        };

        Publisher {
            client: client.clone(),
        }
        .spawn(rx, topic_fmt);

        client
    }

    pub async fn publish<P>(&self, topic: &str, retain: bool, payload: P) -> Result<(), ClientError>
    where
        P: Into<Vec<u8>>,
    {
        self.client
            .publish(topic, QoS::AtLeastOnce, retain, payload)
            .await?;
        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), ClientError> {
        self.client.subscribe(topic, QoS::AtLeastOnce).await?;
        Ok(())
    }

    pub async fn register_button(
        &self,
        button: proto::button::Button,
    ) -> Result<Button, ClientError> {
        let cmd_topic = self.topic_fmt.button_cmd_topic(&button.object_id);
        let discover_topic = self
            .topic_fmt
            .hass_discovery_topic("button", &button.object_id);

        let mut device = payload::device::Device::new();
        let mut component = payload::button::Button::new(cmd_topic.clone())
            .with_object_id(button.object_id.to_string());

        if let Some(button_device) = button.device {
            device = device
                .with_identifiers(vec![button_device.object_id.to_string()])
                .with_name(button_device.name);
            component = component.with_device(&device);
        }

        let button_state = ButtonState::new(button.object_id.clone(), button.on_press);

        {
            let mut state = self.state.lock().await;
            state
                .buttons
                .insert(button.object_id.clone(), button_state.clone());
        }

        self.publish(&discover_topic, false, payload::Json(component))
            .await?;
        self.subscribe(&cmd_topic).await?;

        Ok(Button::new(button_state))
    }

    pub async fn register_switch(
        &self,
        switch: proto::switch::Switch,
    ) -> Result<Switch, ClientError> {
        let cmd_topic = self.topic_fmt.switch_cmd_topic(&switch.object_id);
        let state_topic = self.topic_fmt.sensor_state_topic(&switch.object_id);
        let discover_topic = self
            .topic_fmt
            .hass_discovery_topic("switch", &switch.object_id);

        let mut device = payload::device::Device::new();
        let mut component = payload::switch::Switch::new(cmd_topic.clone())
            .with_object_id(switch.object_id.to_string())
            .with_state_topic(state_topic)
            .with_payload_on(PAYLOAD_ON)
            .with_payload_off(PAYLOAD_OFF)
            .with_unique_id(switch.object_id.to_string());

        if let Some(name) = switch.name {
            component = component.with_name(name);
        }
        if let Some(switch_device) = switch.device {
            device = device
                .with_identifiers(vec![switch_device.object_id.to_string()])
                .with_name(switch_device.name);
            component = component.with_device(device);
        }

        let switch_state =
            SwitchState::new(switch.object_id, PAYLOAD_OFF.to_owned(), self.tx.clone());

        {
            let mut state = self.state.lock().await;
            state
                .switches
                .insert(switch_state.object_id().clone(), switch_state.clone());
        }

        self.publish(&discover_topic, false, payload::Json(component))
            .await?;
        self.subscribe(&cmd_topic).await?;

        Ok(Switch::new(switch_state))
    }

    pub async fn register_number(
        &self,
        number: proto::number::Number,
    ) -> Result<Number, ClientError> {
        let cmd_topic = self.topic_fmt.number_cmd_topic(&number.object_id);
        let state_topic = self.topic_fmt.sensor_state_topic(&number.object_id);
        let discover_topic = self
            .topic_fmt
            .hass_discovery_topic("number", &number.object_id);

        let mut device = payload::device::Device::new();
        let mut component =
            payload::number::Number::new(cmd_topic.clone(), number.unit_of_measurement)
                .with_object_id(number.object_id.to_string())
                .with_state_topic(state_topic)
                .with_unique_id(number.object_id.to_string())
                .with_mode(payload::number::NumberMode::Box);
        if let Some(name) = number.name {
            component = component.with_name(name);
        }
        if let Some(number_device) = number.device {
            device = device
                .with_identifiers(vec![number_device.object_id.to_string()])
                .with_name(number_device.name);
            component = component.with_device(&device);
        }

        let number_state = NumberState::new(number.object_id, number.value, self.tx.clone());

        {
            let mut state = self.state.lock().await;
            state
                .numbers
                .insert(number_state.object_id().clone(), number_state.clone());
        }

        self.publish(&discover_topic, false, payload::Json(component))
            .await?;
        self.subscribe(&cmd_topic).await?;

        Ok(Number::new(number_state))
    }
}

use super::{
    entities::{
        button::{Button, ButtonState},
        number::{Number, NumberState},
        proto,
        switch::{Switch, SwitchState, PAYLOAD_OFF, PAYLOAD_ON},
    },
    message::Message,
    payload,
    publisher::Publisher,
    state::{SharedState, State},
    subscriber::Subscriber,
    topic::TopicFormatter,
};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};
