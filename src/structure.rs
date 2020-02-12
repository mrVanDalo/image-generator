use crate::objects::Line;
use crate::objects::Object;
use crate::objects::Placement;
use crate::objects::Point;
use crate::objects::Query;
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
    #[serde(default)]
    pub compositions: Vec<Placement>,
    #[serde(default)]
    pub objects: HashMap<String, Object>,
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
}

// $> units degree radian
#[inline(always)]
fn degree_to_radian(degree: f64) -> f64 {
    degree * 0.017453293
}

pub trait Querable {
    fn get_element_from_query(&self, query: &Query) -> Option<Box<&dyn Rendable>>;
}

impl Querable for Structure {
    fn get_element_from_query(&self, query: &Query) -> Option<Box<&dyn Rendable>> {
        match &query {
            Query::ByName(name) => match self.objects.get(name) {
                None => None,
                Some(found) => match found {
                    Object::Line(element) => Some(Box::new(element)),
                    Object::Icon(element) => Some(Box::new(element)),
                    Object::Sequence(element) => Some(Box::new(element)),
                    Object::Placement(element) => Some(Box::new(element)),
                },
            },
        }
    }
}

impl Rendable for Structure {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        for placement in &self.compositions {
            context.save();

            context.translate(placement.x, placement.y);
            context.rotate(degree_to_radian(placement.angle));

            context.scale(0.01 * placement.size(), 0.01 * placement.size());

            let rendable = self.get_element_from_query(&placement.query);
            if rendable.is_some() {
                rendable.unwrap().render(&context, querable);
            }

            context.restore();
        }
    }
}
