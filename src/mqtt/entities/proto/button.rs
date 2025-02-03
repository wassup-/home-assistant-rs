pub type OnPress = Box<dyn Fn() + Send + Sync + 'static>;

pub struct Button {
    pub object_id: ObjectId,
    pub name: Option<String>,
    pub device: Option<Device>,
    pub on_press: OnPress,
}

use super::device::Device;
use crate::mqtt::ObjectId;
