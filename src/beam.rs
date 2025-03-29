use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beam {
    pub width: f32,
    pub height: f32,
}

impl Beam {
    pub fn new(width: f32, height: f32) -> Self {
        Beam { width, height }
    }
}
