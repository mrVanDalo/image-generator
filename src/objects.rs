use crate::rendable::Rendable;
use crate::structure::Querable;
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
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    #[serde(rename = "placement")]
    Placement(Placement),

    // empty drawing elements
    // - color
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "spline")]
    Spline(Spline),
    #[serde(rename = "ring")]
    Ring(Ring),

    // filled drawing elements
    // - color
    #[serde(rename = "circle")]
    Circle(Circle),
    #[serde(rename = "icon")]
    Icon(Icon),
}

#[derive(Serialize, Deserialize)]
pub struct Sequence {
    objects: Vec<Object>,
    #[serde(default)]
    pub angle: f64,
    #[serde(default = "Placement::default_size")]
    pub size: f64,
}

impl Rendable for Sequence {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        context.save();
        context.rotate(degree_to_radian(self.angle));
        context.scale(0.01 * self.size, 0.01 * self.size);

        for object in self.objects.iter() {
            match object {
                Object::Circle(element) => element.render(&context, querable),
                Object::Icon(element) => element.render(&context, querable),
                Object::Line(element) => element.render(&context, querable),
                Object::Placement(element) => element.render(&context, querable),
                Object::Ring(element) => element.render(&context, querable),
                Object::Sequence(element) => element.render(&context, querable),
                Object::Spline(element) => element.render(&context, querable),
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
    fn render(&self, context: &Context, querable: &dyn Querable) {
        context.save();

        context.translate(self.x, self.y);

        context.rotate(degree_to_radian(self.angle));
        context.scale(0.01 * self.size, 0.01 * self.size);

        let rendable = querable.get_element_from_query(&self.query);
        if rendable.is_some() {
            rendable.unwrap().render(&context, querable);
        }

        context.restore();
    }
}

#[derive(Serialize, Deserialize)]
pub enum Query {
    #[serde(rename = "by_name")]
    ByName(String),
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    #[serde(default)]
    pub a: Point,
    #[serde(default)]
    pub b: Point,
    #[serde(default = "Color::default")]
    pub color: Color,
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
    fn render(&self, context: &Context, querable: &dyn Querable) {
        self.configure_color(&self.color, context, querable);

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
}

impl Rendable for Spline {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        self.configure_color(&self.color, context, querable);

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
}

impl Ring {
    fn default_radius() -> f64 {
        50.0
    }
}

impl Rendable for Ring {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        self.configure_color(&self.color, context, querable);
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
}

impl Circle {
    fn default_radius() -> f64 {
        50.0
    }
}

impl Rendable for Circle {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        self.configure_color(&self.color, context, querable);
        context.arc(0.0, 0.0, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    path: Vec<Vec<f64>>,
    #[serde(default = "Color::default")]
    pub color: Color,
}

impl Rendable for Icon {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        self.configure_color(&self.color, context, querable);

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
