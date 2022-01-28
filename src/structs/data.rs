use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    pub sensor_id: String,
    pub sender_mac: String,
    pub receiver_mac: String,
    pub rssi: i8,
    pub timestamp: u64
}

impl PartialEq for Datum {
    fn eq(&self, other: &Datum) -> bool {
        self.sensor_id == other.sensor_id && self.sender_mac == other.sender_mac
            && self.receiver_mac == other.receiver_mac
            && self.rssi == other.rssi && self.timestamp == other.timestamp
        // self.x == other.x && self.y == other.y && self.z == other.z
    }
}


pub type Data = VecDeque<Datum>;
