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

pub type Data = VecDeque<Datum>;
