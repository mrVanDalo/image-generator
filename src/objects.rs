//!# Everything you can name in `input.json`
//!
//!These are alle the objects you can use
//!in the `input.json`
//!
//!Everything in the Object enum can be used in the `objects` field
//!of you `input.json`.
//!
//!
//!
//!

use crate::rendable::Rendable;
use crate::structure::ImageContext;
use crate::structure::Query;
use cairo::Context;
use serde::{Deserialize, Serialize};

/// helper function
#[inline(always)]
fn degree_to_radian(degree: f64) -> f64 {
    degree * 0.017453293
}

/// Configures the color to use from the palette to draw.
/// (default is `fill`)
#[derive(Serialize, Deserialize)]
pub enum Color {
    /// Use the background color from the palette to draw.
    #[serde(rename = "background")]
    Background,

    /// Use the `fill` color from the palette to draw.
    #[serde(rename = "fill")]
    Fill,
}

impl Color {
    pub fn default() -> Color {
        Color::Fill
    }
}

/// All object types which can be used
/// in your `input.json`.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    /// A container to draw multiple objects in row.
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    /// alias for `sequence`
    #[serde(rename = "seq")]
    Seq(Sequence),

    /// A container to draw objects in a circle.
    #[serde(rename = "sun")]
    Sun(Sun),

    /// A container to draw multiple objects in a grid.
    #[serde(rename = "grid")]
    Grid(Grid),

    /// draw an unfilled ring (for filling use `circle`)
    #[serde(rename = "ring")]
    Ring(Ring),

    /// draw a filled circle (which is filled)
    #[serde(rename = "circle")]
    Circle(Circle),

    /// draw a path an fill it
    #[serde(rename = "icon")]
    Icon(Icon),

    /// draw a path
    #[serde(rename = "line")]
    Line(Line),
}

impl Object {
    /// extracts tag value from Objects
    pub fn get_tags(&self) -> &Vec<String> {
        match &self {
            Object::Circle(element) => &element.tags,
            Object::Grid(element) => &element.tags,
            Object::Icon(element) => &element.tags,
            Object::Line(element) => &element.tags,
            Object::Ring(element) => &element.tags,
            Object::Sequence(element) => &element.tags,
            Object::Seq(element) => &element.tags,
            Object::Sun(element) => &element.tags,
        }
    }
}

/// A container to draw multiple objects in row.
/// Also useful if you want to draw an object with a
/// different angle or with another center or scale.
///
/// # Example
///
/// ```json
/// {
///  "type": "sequence",
///  "objects": [
///    {"type":"line", "a":{"x":-50,"y":-50}, "b":{"x":50,"y":50}}
///    {"type":"line", "a":{"x":-50,"y":50}, "b":{"x":50,"y":-50}}
///  ],
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Sequence {
    /// list of objects to be drawn
    objects: Vec<Object>,

    /// angle (in degree) to rotate (default is 0)
    #[serde(default)]
    pub angle: f64,

    /// rescale (default is 100 which means no resizing)
    #[serde(default = "Sequence::default_scale")]
    pub scale: f64,

    /// x coordinate of center (default is 0)
    #[serde(default)]
    pub x: f64,

    /// y coordinate of center (default is 0)
    #[serde(default)]
    pub y: f64,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Sequence {
    fn default_scale() -> f64 {
        100.0
    }
}

impl Rendable for Sequence {
    fn render(&self, context: &Context, image_context: &ImageContext, depth: i32) {
        context.save();

        context.translate(self.x, self.y);
        context.rotate(degree_to_radian(self.angle));
        context.scale(0.01 * self.scale, 0.01 * self.scale);

        // stop rendering when scale is to small
        let (x0, y0) = context.user_to_device_distance(100.0, 100.0);
        let (x1, y1) = context.user_to_device_distance(0.0, 0.0);
        if f64::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2)) < 0.3 {
            return ();
        }

        for object in self.objects.iter() {
            match object {
                Object::Circle(element) => element.render(&context, image_context, depth),
                Object::Grid(element) => element.render(&context, image_context, depth),
                Object::Icon(element) => element.render(&context, image_context, depth),
                Object::Line(element) => element.render(&context, image_context, depth),
                Object::Ring(element) => element.render(&context, image_context, depth),
                Object::Sequence(element) => element.render(&context, image_context, depth),
                Object::Seq(element) => element.render(&context, image_context, depth),
                Object::Sun(element) => element.render(&context, image_context, depth),
            }
        }
        context.restore();
    }
}

/// A container to draw multiple objects in a grid.
///
/// You can use it to not repeating yourself over the same object,
/// but also to randomize you picture, by allowing your query to
/// fit on more than one object. In that case one of the objects will be
/// chosen randomly.
///
/// Be aware that like all objects this one centers as well.
/// and so odd and even numbers of rows will result in different
/// object placement.
///
/// # Example
///
/// ```json
/// {
///  "type": "grid",
///  "query": { by_tag: ["number"]}
///  "columns": 10,
///  "width": 45,
/// }
/// ```
///
#[derive(Serialize, Deserialize)]
pub struct Grid {
    /// number of rows
    #[serde(default = "Grid::default_rows")]
    pub rows: i32,

    /// number of columns
    #[serde(default = "Grid::default_columns")]
    pub columns: i32,

    /// width between objects in the grid (default is 100)
    #[serde(default = "Grid::default_width")]
    pub width: f64,

    /// height between objects in the grid (default is 100)
    #[serde(default = "Grid::default_height")]
    pub height: f64,

    /// the query used to find the object which should be placed.
    pub query: Query,

    /// angle (in degree) to rotate (default is 0)
    #[serde(default)]
    pub angle: f64,

    /// rescale (default is 100 which means no resizing)
    #[serde(default = "Grid::default_scale")]
    pub scale: f64,

    /// x coordinate of center (default is 0)
    #[serde(default)]
    pub x: f64,

    /// y coordinate of center (default is 0)
    #[serde(default)]
    pub y: f64,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Grid {
    fn default_scale() -> f64 {
        100.0
    }
    fn default_width() -> f64 {
        100.0
    }
    fn default_height() -> f64 {
        100.0
    }
    fn default_rows() -> i32 {
        1
    }
    fn default_columns() -> i32 {
        1
    }
}

impl Rendable for Grid {
    fn render(&self, context: &Context, image_context: &ImageContext, depth: i32) {
        // stop rendering when scale is to small
        let (x0, y0) = context.user_to_device_distance(100.0, 100.0);
        let (x1, y1) = context.user_to_device_distance(0.0, 0.0);
        if f64::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2)) < 0.3 {
            return ();
        }

        context.save();
        context.translate(self.x, self.y);
        context.rotate(degree_to_radian(self.angle));

        let total_width = self.width * f64::from(self.columns);
        let total_height = self.height * f64::from(self.rows);

        let start_x = (total_width / 2.0) - (self.width / 2.0);
        let start_y = (total_height / 2.0) - (self.height / 2.0);

        for x in (std::ops::Range {
            start: 0,
            end: self.columns,
        }) {
            for y in (std::ops::Range {
                start: 0,
                end: self.rows,
            }) {
                let rendable = image_context.get_element_from_query(&self.query, depth);
                if rendable.is_some() {
                    context.save();
                    context.translate(
                        f64::from(x) * self.width - start_x,
                        f64::from(y) * self.height - start_y,
                    );
                    context.scale(0.01 * self.scale, 0.01 * self.scale);
                    rendable.unwrap().render(&context, image_context, depth - 1);
                    context.restore();
                }
            }
        }

        context.restore();
    }
}

/// A container to draw there objects in a circle.
/// useful to draw a sun
///
/// # Example
///
/// ```json
/// {
///  "type": "sun",
///  "segments": 8,
///  "query": {"by_tag":["arrows"]}
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Sun {
    /// the query used to find the object which should be placed.
    pub query: Query,

    /// radius of the sun distance from center to the center of the objects
    /// (default is 100)
    #[serde(default = "Sun::default_radius")]
    pub radius: f64,

    /// segments of the circle, or how many beams of light does the sun have.
    /// (default is 10)
    #[serde(default = "Sun::default_segments")]
    pub segments: i32,

    /// angle (in degree) to rotate (default is 0)
    #[serde(default)]
    pub angle: f64,

    /// rescale (default is 100 which means no resizing)
    #[serde(default = "Sun::default_scale")]
    pub scale: f64,

    /// x coordinate of center (default is 0)
    #[serde(default)]
    pub x: f64,

    /// y coordinate of center (default is 0)
    #[serde(default)]
    pub y: f64,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Sun {
    fn default_scale() -> f64 {
        100.0
    }
    fn default_radius() -> f64 {
        100.0
    }
    fn default_segments() -> i32 {
        8
    }
}

impl Rendable for Sun {
    fn render(&self, context: &Context, image_context: &ImageContext, depth: i32) {
        context.save();

        context.translate(self.x, self.y);
        context.rotate(degree_to_radian(self.angle));

        // stop rendering when scale is to small
        let (x0, y0) = context.user_to_device_distance(100.0, 100.0);
        let (x1, y1) = context.user_to_device_distance(0.0, 0.0);
        if f64::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2)) < 0.3 {
            return ();
        }

        let segment_rotation_factor = (2.0 * std::f64::consts::PI) / f64::from(self.segments);
        for segment in (std::ops::Range {
            start: 0,
            end: self.segments,
        }) {
            context.save();

            context.rotate(f64::from(segment) * segment_rotation_factor);
            context.translate(self.radius, 0.0);
            context.rotate(degree_to_radian(90.0));
            context.scale(0.01 * self.scale, 0.01 * self.scale);

            let rendable = image_context.get_element_from_query(&self.query, depth);
            if rendable.is_some() {
                rendable.unwrap().render(&context, image_context, depth - 1);
            }

            context.restore();
        }

        context.restore();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

/// draw a ring (without filling)
///
/// #Example
///
/// ```json
/// {
///  "type": "ring",
///  "radius":50,
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Ring {
    /// the radius of the ring (default is 50)
    #[serde(default = "Ring::default_radius")]
    pub radius: f64,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Ring {
    fn default_radius() -> f64 {
        50.0
    }
}

impl Rendable for Ring {
    fn render(&self, context: &Context, image_context: &ImageContext, _depth: i32) {
        self.configure_color(&self.color, context, image_context);
        context.arc(0.0, 0.0, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        self.stroke_and_preserve_line_width(&context);
    }
}

/// draw a circle with filling
///
/// #Example
///
/// ```json
/// {
///  "type": "circle",
///  "radius":50,
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Circle {
    /// the radius of the ring (default is 50)
    #[serde(default = "Circle::default_radius")]
    pub radius: f64,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Circle {
    fn default_radius() -> f64 {
        50.0
    }
}

impl Rendable for Circle {
    fn render(&self, context: &Context, image_context: &ImageContext, _depth: i32) {
        self.configure_color(&self.color, context, image_context);
        context.arc(0.0, 0.0, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
    }
}

/// draw an icon
/// it always is filled with color.
///
/// #Example
///
/// ```json
/// {
///  "type": "icon",
///  "path":[
///  { x: 0, y: 0},
///  { x: 0, y: 0, sa: {x: 0, y:0}, sb:{x: 0, y:0} },
///  ]
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// path to draw the
    path: Vec<IconPoint>,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Rendable for Icon {
    fn render(&self, context: &Context, image_context: &ImageContext, _depth: i32) {
        self.configure_color(&self.color, context, image_context);

        let mut first = true;
        for path in self.path.iter() {
            if first {
                context.move_to(path.x, path.y);
                first = false;
            } else {
                if path.sa.is_none() && path.sb.is_none() {
                    context.line_to(path.x, path.y);
                } else {
                    let sb = if path.sb.is_some() {
                        Point {
                            x: path.sb.as_ref().unwrap().x,
                            y: path.sb.as_ref().unwrap().y,
                        }
                    } else {
                        Point {
                            x: path.x,
                            y: path.y,
                        }
                    };
                    let sa = if path.sa.is_some() {
                        Point {
                            x: path.sa.as_ref().unwrap().x,
                            y: path.sa.as_ref().unwrap().y,
                        }
                    } else {
                        Point {
                            x: path.x,
                            y: path.y,
                        }
                    };
                    context.curve_to(sa.x, sa.y, sb.x, sb.y, path.x, path.y);
                };
            }
        }
        context.close_path();
        context.fill();
    }
}

#[derive(Serialize, Deserialize)]
pub struct IconPoint {
    /// x coordinate
    #[serde(default)]
    pub x: f64,

    /// y coordinate
    #[serde(default)]
    pub y: f64,

    /// spline point of source
    sa: Option<Point>,

    /// spline point of target (the point given by x and y)
    sb: Option<Point>,
}

/// draw an path
/// it always is just the line with color.
///
/// #Example
///
/// ```json
/// {
///  "type": "path",
///  "path":[
///  { x: 0, y: 0},
///  { x: 0, y: 0, sa: {x: 0, y:0}, sb:{x: 0, y:0} },
///  ]
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Line {
    /// path to draw the
    path: Vec<IconPoint>,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Rendable for Line {
    fn render(&self, context: &Context, image_context: &ImageContext, _depth: i32) {
        self.configure_color(&self.color, context, image_context);

        let mut first = true;
        for path in self.path.iter() {
            if first {
                context.move_to(path.x, path.y);
                first = false;
            } else {
                if path.sa.is_none() && path.sb.is_none() {
                    context.line_to(path.x, path.y);
                } else {
                    let sb = if path.sb.is_some() {
                        Point {
                            x: path.sb.as_ref().unwrap().x,
                            y: path.sb.as_ref().unwrap().y,
                        }
                    } else {
                        Point {
                            x: path.x,
                            y: path.y,
                        }
                    };
                    let sa = if path.sa.is_some() {
                        Point {
                            x: path.sa.as_ref().unwrap().x,
                            y: path.sa.as_ref().unwrap().y,
                        }
                    } else {
                        Point {
                            x: path.x,
                            y: path.y,
                        }
                    };
                    context.curve_to(sa.x, sa.y, sb.x, sb.y, path.x, path.y);
                };
            }
        }
        self.stroke_and_preserve_line_width(context);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::structure::Structure;
    use cairo::Context;
    use cairo::Format;
    use cairo::ImageSurface;
    use serde_json::json;

    #[test]
    fn grid_recursion_always_terminates() {
        // create a structure that loops for ever
        // but should stop after a while
        let input = json!({
            "start": {"by_name":"main"},
            "objects": {
                "main":{
                    "type":"grid",
                    "query":{"by_name":"main"},
                }
            }
        });
        let result = Structure::load_from_value(input);
        assert!(result.is_ok());
        let structure = result.unwrap();
        let image_context = ImageContext::new(&structure);
        let surface = ImageSurface::create(Format::Rgb24, 100, 100).expect("Can't create surface");
        let context = Context::new(&surface);
        structure.render(&context, &image_context, 100);
        // if this function is not crashing, than all good
    }

    #[test]
    fn sun_recursion_always_terminates() {
        // create a structure that loops for ever
        // but should stop after a while
        let input = json!({
            "start": {"by_name":"main"},
            "objects": {
                "main":{
                    "type":"sun",
                    "segments":1,
                    "query":{"by_name":"main"},
                }
            }
        });
        let result = Structure::load_from_value(input);
        assert!(result.is_ok());
        let structure = result.unwrap();
        let image_context = ImageContext::new(&structure);
        let surface = ImageSurface::create(Format::Rgb24, 100, 100).expect("Can't create surface");
        let context = Context::new(&surface);
        structure.render(&context, &image_context, 100);
        // if this function is not crashing, than all good
    }
}
