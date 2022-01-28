use std::collections::{VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::structs::data::Data;


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

    pub fn get_device_data(
        &self,
        device_id: String
    ) -> Data {
        let mut data = self.data.read().clone();
        data.retain(|datum| datum.sender_mac == device_id);
        data
    }
}
