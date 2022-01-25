use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Datum {
    name: String,
    quantity: i32,
}

type Data = VecDeque<Datum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sensor {
    pub id: String,
    pos: Position
}

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
