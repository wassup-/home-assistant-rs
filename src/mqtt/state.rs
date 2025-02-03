#[derive(Default)]
pub struct State {
    pub buttons: HashMap<ObjectId, ButtonState>,
    pub numbers: HashMap<ObjectId, NumberState>,
    pub switches: HashMap<ObjectId, SwitchState>,
}

pub type SharedState = Arc<Mutex<State>>;

use super::{
    entities::{button::ButtonState, number::NumberState, switch::SwitchState},
    ObjectId,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
