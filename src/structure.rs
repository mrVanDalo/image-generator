//use crate::objects::Line;
use crate::objects::Object;
//use crate::objects::Placement;
//use crate::objects::Point;
//use crate::objects::Query;
use crate::palette::Palette;
use crate::rendable::Rendable;
use cairo::Context;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Structure {
    #[serde(default = "Structure::default_width")]
    pub width: i32,
    #[serde(default = "Structure::default_height")]
    pub height: i32,
    #[serde(default)]
    pub objects: HashMap<String, Object>,
    pub start: Query,
}

impl Structure {
    fn default_width() -> i32 {
        100
    }
    fn default_height() -> i32 {
        100
    }
    pub fn load_from_file(path: &str) -> Result<Structure, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let structure: Structure = serde_json::from_reader(reader)?;
        return Ok(structure);
    }
}

impl Rendable for Structure {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        let rendable = image_context.get_element_from_query(&self.start);
        if rendable.is_some() {
            rendable.unwrap().render(&context, image_context);
        }
    }
}

// -------

#[derive(Serialize, Deserialize)]
pub enum Query {
    #[serde(rename = "by_name")]
    ByName(String),
    #[serde(rename = "one_of_names")]
    OneOfNames(Vec<String>),
    #[serde(rename = "by_tag")]
    ByTag(Vec<String>),
}

pub struct ImageContext<'a> {
    pub objects: &'a HashMap<String, Object>,
    pub tags: HashMap<&'a String, Vec<&'a Object>>,
    pub palette: Palette,
}

impl ImageContext<'_> {
    pub fn new(objects: &HashMap<String, Object>) -> ImageContext {
        let mut tags_map: HashMap<&String, Vec<&Object>> = HashMap::new();
        for object in objects.values() {
            let tags = object.get_tags();
            for tag in tags.iter() {
                match tags_map.get_mut(tag) {
                    None => {
                        tags_map.insert(tag, vec![object]);
                    }
                    Some(entry) => {
                        entry.push(object);
                    }
                }
            }
        }
        ImageContext {
            objects: &objects,
            tags: tags_map,
            palette: Palette::dark_on_bright(Palette::random_color()),
        }
    }

    fn object_to_rendable_box(object: &Object) -> Option<Box<&dyn Rendable>> {
        match object {
            Object::Circle(element) => Some(Box::new(element)),
            Object::Icon(element) => Some(Box::new(element)),
            Object::Line(element) => Some(Box::new(element)),
            Object::Placement(element) => Some(Box::new(element)),
            Object::Ring(element) => Some(Box::new(element)),
            Object::Sequence(element) => Some(Box::new(element)),
            Object::Spline(element) => Some(Box::new(element)),
        }
    }

    // todo : abort when scale is to small
    pub fn get_element_from_query(&self, query: &Query) -> Option<Box<&dyn Rendable>> {
        match &query {
            Query::ByName(name) => match self.objects.get(name) {
                None => None,
                Some(found) => ImageContext::object_to_rendable_box(found),
            },
            Query::OneOfNames(names) => match names.choose(&mut rand::thread_rng()) {
                None => None,
                Some(name) => self.get_element_from_query(&Query::ByName(name.to_string())),
            },
            Query::ByTag(tags) => match tags.choose(&mut rand::thread_rng()) {
                None => None,
                Some(tag) => match self.tags.get(tag) {
                    None => None,
                    Some(objects) => match objects.choose(&mut rand::thread_rng()) {
                        None => None,
                        Some(object) => ImageContext::object_to_rendable_box(object),
                    },
                },
            },
        }
    }
    pub fn palette(&self) -> &Palette {
        &self.palette
    }
}
