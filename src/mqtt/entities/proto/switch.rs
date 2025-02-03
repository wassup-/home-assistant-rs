pub struct Switch {
    pub object_id: ObjectId,
    pub name: Option<String>,
    pub device: Option<Device>,
}

use super::device::Device;
use crate::mqtt::ObjectId;
