use serde::Serialize;

pub struct Json<T>(pub T);

impl<T: Serialize> Into<Vec<u8>> for Json<T> {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap()
    }
}

pub mod button {
    pub use ha_mqtt::components::button::Button;
}

pub mod device {
    pub use ha_mqtt::device::Device;
}

pub mod number {
    pub use ha_mqtt::components::number::{Number, NumberMode};
}

pub mod switch {

    #[derive(Serialize)]
    pub struct Switch {
        cmd_topic: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        object_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        state_topic: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload_on: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload_off: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        device: Option<Device>,
        #[serde(skip_serializing_if = "Option::is_none")]
        unique_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    }

    impl Switch {
        pub fn new(cmd_topic: String) -> Self {
            Switch {
                cmd_topic,
                object_id: None,
                state_topic: None,
                payload_on: None,
                payload_off: None,
                device: None,
                unique_id: None,
                name: None,
            }
        }

        pub fn with_object_id(mut self, object_id: impl Into<String>) -> Self {
            self.object_id = Some(object_id.into());
            self
        }

        pub fn with_state_topic(mut self, state_topic: impl Into<String>) -> Self {
            self.state_topic = Some(state_topic.into());
            self
        }

        pub fn with_payload_on(mut self, payload_on: impl Into<String>) -> Self {
            self.payload_on = Some(payload_on.into());
            self
        }

        pub fn with_payload_off(mut self, payload_off: impl Into<String>) -> Self {
            self.payload_off = Some(payload_off.into());
            self
        }

        pub fn with_unique_id(mut self, unique_id: impl Into<String>) -> Self {
            self.unique_id = Some(unique_id.into());
            self
        }

        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        pub fn with_device(mut self, device: Device) -> Self {
            self.device = Some(device);
            self
        }
    }

    use serde::Serialize;

    use super::device::Device;
}
