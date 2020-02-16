use crate::objects::Object;
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
    #[serde(default = "Structure::default_color_scheme")]
    pub color_scheme: ColorScheme,
    #[serde(default = "Structure::default_line_size")]
    pub line_size: f64,
}

#[derive(Serialize, Deserialize)]
pub enum ColorScheme {
    #[serde(rename = "dark_on_bright")]
    DarkOnBright,
    #[serde(rename = "bright_on_dark")]
    BrightOnDark,
}

impl Structure {
    fn default_color_scheme() -> ColorScheme {
        ColorScheme::DarkOnBright
    }
    fn default_width() -> i32 {
        100
    }
    fn default_height() -> i32 {
        100
    }
    fn default_line_size() -> f64 {
        1.0
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
    pub fn new(structure: &Structure) -> ImageContext {
        let mut tags_map: HashMap<&String, Vec<&Object>> = HashMap::new();
        for object in structure.objects.values() {
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

        let palette = match &structure.color_scheme {
            ColorScheme::DarkOnBright => Palette::dark_on_bright(Palette::random_color()),
            ColorScheme::BrightOnDark => Palette::bright_on_dark(Palette::random_color()),
        };

        ImageContext {
            objects: &structure.objects,
            tags: tags_map,
            palette: palette,
        }
    }

    fn object_to_rendable_box(object: &Object) -> Option<Box<&dyn Rendable>> {
        match object {
            Object::Circle(element) => Some(Box::new(element)),
            Object::Grid(element) => Some(Box::new(element)),
            Object::Icon(element) => Some(Box::new(element)),
            Object::Line(element) => Some(Box::new(element)),
            Object::Placement(element) => Some(Box::new(element)),
            Object::Ring(element) => Some(Box::new(element)),
            Object::Sequence(element) => Some(Box::new(element)),
            Object::Spline(element) => Some(Box::new(element)),
            Object::Sun(element) => Some(Box::new(element)),
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
