use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    name: String,
    sender_mac: String,
    receiver_mac: String,
    rssi: i32,
    timestamp: u64
}

pub type Data = VecDeque<Datum>;
