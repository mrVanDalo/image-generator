use crate::rendable::Rendable;
use crate::structure::ImageContext;
use crate::structure::Query;
use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "fill")]
    Fill,
}

impl Color {
    pub fn default() -> Color {
        Color::Fill
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    // containers should have:
    // - angle
    // - size // todo : rename scale
    // - tags
    // - stop when scale is to small
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    #[serde(rename = "placement")]
    Placement(Placement),

    // empty drawing elements
    // - color
    // - tags
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "spline")]
    Spline(Spline),
    #[serde(rename = "ring")]
    Ring(Ring),

    // filled drawing elements
    // - color
    // - tags
    #[serde(rename = "circle")]
    Circle(Circle),
    #[serde(rename = "icon")]
    Icon(Icon),
}

impl Object {
    pub fn get_tags(&self) -> &Vec<String> {
        match &self {
            Object::Circle(element) => &element.tags,
            Object::Icon(element) => &element.tags,
            Object::Line(element) => &element.tags,
            Object::Placement(element) => &element.tags,
            Object::Ring(element) => &element.tags,
            Object::Sequence(element) => &element.tags,
            Object::Spline(element) => &element.tags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sequence {
    objects: Vec<Object>,
    #[serde(default)]
    pub angle: f64,
    #[serde(default = "Placement::default_size")]
    pub size: f64,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Rendable for Sequence {
    fn render(&self, context: &Context, image_context: &ImageContext) {
        context.save();

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
            }
        }
        context.restore();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Placement {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub angle: f64,
    #[serde(default = "Placement::default_size")]
    pub size: f64,
    pub query: Query,
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

#[derive(Serialize, Deserialize)]
pub struct Line {
    #[serde(default)]
    pub a: Point,
    #[serde(default)]
    pub b: Point,
    #[serde(default = "Color::default")]
    pub color: Color,
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

#[derive(Serialize, Deserialize)]
pub struct Spline {
    pub a: Point,
    pub b: Point,
    pub sa: Point,
    pub sb: Point,
    #[serde(default = "Color::default")]
    pub color: Color,
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

#[derive(Serialize, Deserialize)]
pub struct Ring {
    #[serde(default = "Ring::default_radius")]
    pub radius: f64,
    #[serde(default = "Color::default")]
    pub color: Color,
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

#[derive(Serialize, Deserialize)]
pub struct Circle {
    #[serde(default = "Circle::default_radius")]
    pub radius: f64,
    #[serde(default = "Color::default")]
    pub color: Color,
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

#[derive(Serialize, Deserialize)]
pub struct Icon {
    path: Vec<Vec<f64>>,
    #[serde(default = "Color::default")]
    pub color: Color,
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
