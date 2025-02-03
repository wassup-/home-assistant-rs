pub enum Payload {
    SensorState(f64),
}

pub struct Message {
    pub object_id: ObjectId,
    pub payload: Payload,
}

impl Payload {
    pub fn to_bytes(self) -> Vec<u8> {
        todo!()
    }
}

use super::ObjectId;
