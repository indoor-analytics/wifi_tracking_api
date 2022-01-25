use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    name: String,
    quantity: i32,
}

pub type Data = VecDeque<Datum>;