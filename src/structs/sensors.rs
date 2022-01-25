use serde::{Serialize, Deserialize};
use crate::structs::position::Position;
use std::collections::{HashMap};
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sensor {
    pub id: String,
    pos: Position
}

#[derive(Clone)]
pub struct Sensors {
    pub sensors: Arc<RwLock<HashMap<String, Sensor>>>
}
impl Sensors {
    pub fn new() -> Self {
        Sensors {
            sensors: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}
