use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod icon;
use icon::Icon;

mod composition;
use composition::Composition;
use composition::Query;

const DEFAULT_WIDTH: i32 = 100;
const DEFAULT_HEIGHT: i32 = 100;

#[derive(Serialize, Deserialize)]
pub struct Structure {
    width: Option<i32>,
    height: Option<i32>,
    pub icons: HashMap<String, Icon>,
    pub compositions: Vec<Composition>,
}

impl Structure {
    pub fn load_from_file(path: &str) -> Result<Structure, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let structure: Structure = serde_json::from_reader(reader)?;
        return Ok(structure);
    }
    pub fn get_image_width(&self) -> i32 {
        match self.width {
            Some(width) => width,
            None => DEFAULT_WIDTH,
        }
    }
    pub fn get_image_height(&self) -> i32 {
        match self.height {
            Some(height) => height,
            None => DEFAULT_HEIGHT,
        }
    }

    pub fn get_element_from_query(&self, query: &Query) -> Option<&Icon> {
        match &query.icon {
            Some(icon) => self.icons.get(icon),
            _ => None,
        }
    }
}
