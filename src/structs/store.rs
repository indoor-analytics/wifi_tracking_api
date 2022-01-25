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
}