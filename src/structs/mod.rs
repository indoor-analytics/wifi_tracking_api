use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    name: String,
    quantity: i32,
}

type Data = VecDeque<Datum>;

#[derive(Clone)]
pub struct Store {
    pub data: Arc<RwLock<Data>>
}
impl Store {
    pub fn new() -> Self {
        Store {
            data: Arc::new(RwLock::new(VecDeque::new()))
        }
    }
}
