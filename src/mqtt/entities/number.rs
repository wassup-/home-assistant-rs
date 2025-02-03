pub type NumberState = State<f64>;

pub struct Number {
    state: NumberState,
}

impl Number {
    pub fn new(state: NumberState) -> Self {
        Number { state }
    }

    pub async fn set_value(&self, value: f64) {
        self.state.update_value(value).await
    }

    pub async fn publish_value(&self) {
        self.state.publish_value().await
    }
}

use crate::mqtt::entities::State;
