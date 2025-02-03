pub type OnPress = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Clone)]
pub struct ButtonState {
    object_id: ObjectId,
    on_press: Arc<OnPress>,
}

pub struct Button {
    state: ButtonState,
}

impl Button {
    pub fn new(state: ButtonState) -> Self {
        Button { state }
    }

    pub async fn press(&self) {
        self.state.press().await
    }
}

impl ButtonState {
    pub fn new(object_id: ObjectId, on_press: OnPress) -> Self {
        ButtonState {
            object_id,
            on_press: Arc::new(on_press),
        }
    }

    pub fn object_id(&self) -> &ObjectId {
        &self.object_id
    }

    pub async fn press(&self) {
        (self.on_press)()
    }
}

use std::sync::Arc;

use crate::mqtt::ObjectId;
