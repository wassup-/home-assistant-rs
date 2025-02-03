pub type SwitchState = State<String>;

pub const PAYLOAD_ON: &str = "ON";
pub const PAYLOAD_OFF: &str = "OFF";

pub struct Switch {
    state: SwitchState,
}

impl Switch {
    pub fn new(state: SwitchState) -> Self {
        Switch { state }
    }

    pub async fn turn_on(&self) {
        self.state.update_value(PAYLOAD_ON.to_owned()).await
    }

    pub async fn turn_off(&self) {
        self.state.update_value(PAYLOAD_OFF.to_owned()).await
    }

    pub async fn publish_value(&self) {
        self.state.publish_value().await
    }
}

use crate::mqtt::entities::State;
