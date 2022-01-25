use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}
