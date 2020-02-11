use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Composition {
    pub x: f64,
    pub y: f64,
    size: Option<f64>,
    pub query: Query,
}

impl Composition {
    pub fn size(&self) -> f64 {
        match self.size {
            Some(size) => size,
            None => 100.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub icon: Option<String>,
}
