pub struct Number {
    pub object_id: ObjectId,
    pub value: f64,
    pub unit_of_measurement: String,
    pub name: Option<String>,
    pub device: Option<Device>,
}

use super::device::Device;
use crate::mqtt::ObjectId;
