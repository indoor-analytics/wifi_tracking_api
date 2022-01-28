use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    sensor_id: String,
    sender_mac: String,
    receiver_mac: String,
    rssi: i8,
    timestamp: u64
}

pub type Data = VecDeque<Datum>;
