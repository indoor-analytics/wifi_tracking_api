use core::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl fmt::Display for Position {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&*("{\"x\":".to_owned() + &*self.x.to_string() + ",")).ok();
        fmt.write_str(&*("\"y\":".to_owned() + &*self.y.to_string() + ",")).ok();
        fmt.write_str(&*("\"z\":".to_owned() + &*self.z.to_string() + "}")).ok();
        Ok(())
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
