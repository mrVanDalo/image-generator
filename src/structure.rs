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
use std::rc::Rc;

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
        let rendable = image_context
            .get_element_from_query(&self.start, depth)
            .next();
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

impl Query {
    pub fn get_choose(&self) -> &Choose {
        match self {
            Query::ByName { by_name: _, choose } => choose,
            Query::OneOfNames {
                one_of_names: _,
                choose,
            } => choose,
            Query::ByTag { by_tag: _, choose } => choose,
        }
    }
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

    fn object_to_rendable_box(object: &Object) -> Option<Rc<&dyn Rendable>> {
        match object {
            Object::Circle(element) => Some(Rc::new(element)),
            Object::Grid(element) => Some(Rc::new(element)),
            Object::Icon(element) => Some(Rc::new(element)),
            Object::Line(element) => Some(Rc::new(element)),
            Object::Ring(element) => Some(Rc::new(element)),
            Object::Sequence(element) => Some(Rc::new(element)),
            Object::Seq(element) => Some(Rc::new(element)),
            Object::Sun(element) => Some(Rc::new(element)),
        }
    }

    pub fn get_element_from_query<'a>(&'a self, query: &'a Query, depth: i32) -> QueryResult {
        // if to deep just stop with the elements
        if depth < 1 {
            QueryResult {
                objects: &self.objects,
                tags: &self.tags,
                query: query,
                current_item: CurrentItem::Nothing,
                is_dead_end: true,
            }
        } else {
            QueryResult {
                objects: &self.objects,
                tags: &self.tags,
                query: query,
                current_item: CurrentItem::Uninitalized,
                is_dead_end: false,
            }
        }
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }
}

pub struct QueryResult<'a> {
    objects: &'a HashMap<String, Object>,
    tags: &'a HashMap<&'a String, Vec<&'a Object>>,
    query: &'a Query,
    current_item: CurrentItem<'a>,
    is_dead_end: bool,
}

pub enum CurrentItem<'a> {
    Uninitalized,
    Found(Rc<&'a dyn Rendable>),
    Nothing,
}

impl<'a> QueryResult<'a> {
    pub fn query_next(&self) -> Option<Rc<&'a dyn Rendable>> {
        match &self.query {
            Query::ByName {
                by_name: name,
                choose: _,
            } => match self.objects.get(name) {
                None => None,
                Some(found) => ImageContext::object_to_rendable_box(found),
            },
            Query::OneOfNames {
                one_of_names,
                choose: _,
            } => match one_of_names.choose(&mut rand::thread_rng()) {
                None => None,
                Some(name) => match self.objects.get(name) {
                    None => None,
                    Some(found) => ImageContext::object_to_rendable_box(found),
                },
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
}

impl<'a> Iterator for QueryResult<'a> {
    type Item = Rc<&'a dyn Rendable>;
    fn next(&mut self) -> Option<Rc<&'a dyn Rendable>> {
        if self.is_dead_end {
            return None;
        }
        match self.query.get_choose() {
            Choose::Once => match &self.current_item {
                CurrentItem::Uninitalized => match self.query_next() {
                    None => {
                        self.current_item = CurrentItem::Nothing;
                        None
                    }
                    Some(found) => {
                        self.current_item = CurrentItem::Found(found.clone());
                        Some(found)
                    }
                },
                CurrentItem::Found(found) => Some(found.clone()),
                CurrentItem::Nothing => None,
            },
            Choose::EveryTime => self.query_next(),
        }
    }
}
