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

    // containers should have:
    // - angle
    // - size // todo : rename scale
    // - tags
    // - stop when scale is to small

    /// A container to draw multiple objects in row.
    #[serde(rename = "sequence")]
    Sequence(Sequence),

    /// A container to draw another object.
    #[serde(rename = "placement")]
    Placement(Placement),

    /// A container to draw there objects in a circle.
    #[serde(rename = "sun")]
    Sun(Sun),


    // empty drawing elements
    // - color
    // - tags

    /// draw a line
    #[serde(rename = "line")]
    Line(Line),

    /// draw a spline
    #[serde(rename = "spline")]
    Spline(Spline),

    /// draw a ring (for filling use `circle`)
    #[serde(rename = "ring")]
    Ring(Ring),

    // filled drawing elements
    // - color
    // - tags

    /// draw a circle (which is filled)
    #[serde(rename = "circle")]
    Circle(Circle),

    /// draw a path
    #[serde(rename = "icon")]
    Icon(Icon),
}

impl Object {

    /// extracts tag value from Objects
    pub fn get_tags(&self) -> &Vec<String> {
        match &self {
            Object::Circle(element) => &element.tags,
            Object::Icon(element) => &element.tags,
            Object::Line(element) => &element.tags,
            Object::Placement(element) => &element.tags,
            Object::Ring(element) => &element.tags,
            Object::Sequence(element) => &element.tags,
            Object::Spline(element) => &element.tags,
            Object::Sun(element) => &element.tags,
        }
    }
}

/// A container to draw multiple objects in row.
/// Also useful if you want to draw an object with a
/// different angle or with another center or size.
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

    /// resize (default is 100 which means no resizing)
    #[serde(default = "Placement::default_size")]
    pub size: f64,

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

impl Rendable for Sequence {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        context.save();

        context.translate(self.x, self.y);
        context.rotate(degree_to_radian(self.angle));
        context.scale(0.01 * self.size, 0.01 * self.size);

        // stop rendering when scale is to small
        let (x0, y0) = context.user_to_device_distance(100.0, 100.0);
        let (x1, y1) = context.user_to_device_distance(0.0, 0.0);
        if f64::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2)) < 0.3 {
            return ();
        }

        for object in self.objects.iter() {
            match object {
                Object::Circle(element) => element.render(&context, image_context),
                Object::Icon(element) => element.render(&context, image_context),
                Object::Line(element) => element.render(&context, image_context),
                Object::Placement(element) => element.render(&context, image_context),
                Object::Ring(element) => element.render(&context, image_context),
                Object::Sequence(element) => element.render(&context, image_context),
                Object::Spline(element) => element.render(&context, image_context),
                Object::Sun(element) => element.render(&context, image_context),
            }
        }
        context.restore();
    }
}

/// A container to draw another object.
/// You can use it to not repeating yourself over the same object,
/// but also to randomize you picture, by allowing your query to
/// fit on more than one object. In that case one of the objects will be
/// chosen randomly.
///
/// # Example
///
/// ```json
/// {
///  "type": "placement",
///  "query": { by_tag: ["number"]}
/// }
/// ```
///
/// ```json
/// {
///  "type": "placement",
///  "query": { by_name: "1"}
/// }
/// ```
///
/// ```json
/// {
///  "type": "placement",
///  "query": { one_of_names: ["1","2","3"]}
/// }
/// ```
///
#[derive(Serialize, Deserialize)]
pub struct Placement {

    /// the query used to find the object which should be placed.
    pub query: Query,

    /// angle (in degree) to rotate (default is 0)
    #[serde(default)]
    pub angle: f64,

    /// resize (default is 100 which means no resizing)
    #[serde(default = "Placement::default_size")]
    pub size: f64,

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

impl Placement {
    fn default_size() -> f64 {
        100.0
    }
}

#[inline(always)]
fn degree_to_radian(degree: f64) -> f64 {
    degree * 0.017453293
}

impl Rendable for Placement {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        context.save();

        context.translate(self.x, self.y);
        context.rotate(degree_to_radian(self.angle));
        context.scale(0.01 * self.size, 0.01 * self.size);

        // stop rendering when scale is to small
        let (x0, y0) = context.user_to_device_distance(100.0, 100.0);
        let (x1, y1) = context.user_to_device_distance(0.0, 0.0);
        if f64::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2)) < 0.3 {
            return ();
        }

        let rendable = image_context.get_element_from_query(&self.query);
        if rendable.is_some() {
            rendable.unwrap().render(&context, image_context);
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

    /// resize (default is 100 which means no resizing)
    #[serde(default = "Sun::default_size")]
    pub size: f64,

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
    fn default_size() -> f64 {
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
    fn render(&self, context: &Context, image_context: &ImageContext) {
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
            context.scale(0.01 * self.size, 0.01 * self.size);

            let rendable = image_context.get_element_from_query(&self.query);
            if rendable.is_some() {
                rendable.unwrap().render(&context, image_context);
            }

            context.restore();
        }

        context.restore();
    }
}

/// draw a Line
///
/// #Example
///
/// ```json
/// {
///  "type": "line",
///  "a":{"x":50,"y":50},
///  "b":{"x":-50,"y":-50},
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Line {

    /// point to start drawing
    #[serde(default)]
    pub a: Point,

    /// point to stop drawing
    #[serde(default)]
    pub b: Point,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
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

impl Rendable for Line {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        self.configure_color(&self.color, context, image_context);

        // draw line
        context.move_to(self.a.x, self.a.y);
        context.line_to(self.b.x, self.b.y);

        self.stroke_and_preserve_line_width(context);
    }
}

/// draw a spline
///
/// #Example
///
/// ```json
/// {
///  "type": "spline",
///  "a":{"x":50,"y":50},
///  "sa":{"x":0,"y":50},
///  "b":{"x":-50,"y":-50},
///  "sb":{"x":0,"y":-50},
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Spline {

    /// point to start drawing
    pub a: Point,

    /// point to stop drawing
    pub b: Point,

    /// point to draw the line to at the end of `a`
    pub sa: Point,

    /// point to draw the line to at the end of `b`
    pub sb: Point,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Rendable for Spline {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        self.configure_color(&self.color, context, image_context);

        // draw line
        context.move_to(self.a.x, self.a.y);
        context.curve_to(
            self.sa.x, self.sa.y, self.sb.x, self.sb.y, self.b.x, self.b.y,
        );

        self.stroke_and_preserve_line_width(context);
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
    fn render(&self, context: &Context, image_context: &ImageContext) {
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
    fn render(&self, context: &Context, image_context: &ImageContext) {
        self.configure_color(&self.color, context, image_context);
        context.arc(0.0, 0.0, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
    }
}

/// draw a path/icon
/// it always is filled with color.
///
/// #Example
///
/// ```json
/// {
///  "type": "icon",
///  "path":[
///  [0,0],[50,50],[-50,50]
///  ]
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// path to draw the
    path: Vec<Vec<f64>>,

    /// color from the palette to draw with
    #[serde(default = "Color::default")]
    pub color: Color,

    /// tags of this object which can be used to query.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Rendable for Icon {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        self.configure_color(&self.color, context, image_context);

        let mut first = true;
        for path in self.path.iter() {
            if first {
                context.move_to(path[0], path[1]);
                first = false;
            } else {
                if path.len() == 2 {
                    context.line_to(path[0], path[1]);
                } else if path.len() == 6 {
                    context.curve_to(path[0], path[1], path[2], path[3], path[4], path[5]);
                }
                // todo
                // else return error
            }
        }
        context.close_path();

        context.fill();
    }
}
