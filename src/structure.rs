use crate::objects::Object;
use crate::palette::Palette;
use crate::rendable::Rendable;
use cairo::Context;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Structure {
    /// width of the picture
    #[serde(default = "Structure::default_width")]
    pub width: i32,

    /// height of the picture
    #[serde(default = "Structure::default_height")]
    pub height: i32,

    /// List of defined objects which can be drawn.
    #[serde(default)]
    pub objects: HashMap<String, Object>,

    /// Query to find the first element to draw from
    pub start: Query,

    /// color scheme to use for generating palette
    #[serde(default = "Structure::default_color_scheme")]
    pub color_scheme: ColorScheme,

    /// how thick should lines be drawn
    /// this will be not affected by scaling size
    #[serde(default = "Structure::default_line_size")]
    pub line_size: f64,

    /// How deep should recursion go?
    /// How many queries in a row should be called before stopping.
    #[serde(default = "Structure::default_depth")]
    pub depth: i32,
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
    fn default_depth() -> i32 {
        30
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
    pub fn load_from_value(input: Value) -> Result<Structure, Box<dyn Error>> {
        let structure: Structure = serde_json::from_value(input)?;
        return Ok(structure);
    }
}

impl Rendable for Structure {
    fn render(&self, context: &Context, image_context: &ImageContext, depth: i32) {
        let rendable = image_context.get_element_from_query(&self.start, depth);
        if rendable.is_some() {
            rendable.unwrap().render(&context, image_context, depth);
        }
    }
}

// -------

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query {
    ByName {
        by_name: String,
        #[serde(default)]
        choose: Choose,
    },
    OneOfNames {
        one_of_names: Vec<String>,
        #[serde(default)]
        choose: Choose,
    },
    ByTag {
        by_tag: Vec<String>,
        #[serde(default)]
        choose: Choose,
    },
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Choose {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "every_time")]
    EveryTime,
}

impl Default for Choose {
    fn default() -> Self {
        Choose::EveryTime
    }
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
            Object::Ring(element) => Some(Box::new(element)),
            Object::Sequence(element) => Some(Box::new(element)),
            Object::Seq(element) => Some(Box::new(element)),
            Object::Sun(element) => Some(Box::new(element)),
        }
    }

    // todo : abort when scale is to small
    pub fn get_element_from_query(&self, query: &Query, depth: i32) -> Option<Box<&dyn Rendable>> {
        // if to deep just stop with the elements
        if depth < 1 {
            return None;
        }
        match &query {
            Query::ByName {
                by_name: name,
                choose: _,
            } => match self.objects.get(name) {
                None => None,
                Some(found) => ImageContext::object_to_rendable_box(found),
            },
            Query::OneOfNames {
                one_of_names,
                choose,
            } => match one_of_names.choose(&mut rand::thread_rng()) {
                None => None,
                Some(name) => self.get_element_from_query(
                    &Query::ByName {
                        by_name: name.to_string(),
                        choose: *choose,
                    },
                    depth,
                ),
            },
            Query::ByTag {
                by_tag: tags,
                choose: _,
            } => match tags.choose(&mut rand::thread_rng()) {
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
