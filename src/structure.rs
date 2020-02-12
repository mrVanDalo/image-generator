use crate::composition::Composition;
use crate::composition::Placement;
use crate::composition::Query;
use crate::icon::Icon;
use crate::rendable::Rendable;
use cairo::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

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
    #[inline]
    pub fn get_image_width(&self) -> i32 {
        match self.width {
            Some(width) => width,
            None => DEFAULT_WIDTH,
        }
    }
    #[inline]
    pub fn get_image_height(&self) -> i32 {
        match self.height {
            Some(height) => height,
            None => DEFAULT_HEIGHT,
        }
    }
    pub fn get_element_from_query(&self, query: &Query) -> Option<Box<&dyn Rendable>> {
        match &query {
            Query::Icon(icon) => match self.icons.get(icon) {
                None => None,
                Some(icon) => Some(Box::new(icon)),
            },
        }
    }
}

impl Rendable for Structure {
    fn render(&self, context: &Context) {
        for composition in &self.compositions {
            context.save();

            match composition.placement {
                Placement::Absolute { x, y } => context.translate(x, y),
                Placement::Relative { x, y } => context.translate(
                    x / 100.0 * f64::from(self.get_image_width()),
                    y / 100.0 * f64::from(self.get_image_height()),
                ),
            }

            context.scale(0.01 * composition.size(), 0.01 * composition.size());

            let rendable = self.get_element_from_query(&composition.query);
            if rendable.is_some() {
                rendable.unwrap().render(&context);
            }

            context.restore();
        }
    }
}
