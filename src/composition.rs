use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Composition {
    pub placement: Placement,
    size: Option<f64>,
    pub query: Query,
}

#[derive(Serialize, Deserialize)]
pub enum Placement {
    #[serde(rename = "absolute")]
    Absolute {
        x: f64,
        y: f64,
        #[serde(default)]
        angle: f64,
    },
    #[serde(rename = "relative")]
    Relative {
        x: f64,
        y: f64,

        #[serde(default)]
        angle: f64,
    },
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
pub enum Query {
    #[serde(rename = "icon")]
    Icon(String),
}
