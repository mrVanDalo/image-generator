use crate::rendable::Rendable;
use crate::structure::Querable;
use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "circle")]
    Circle(Circle),
    #[serde(rename = "icon")]
    Icon(Icon),
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    #[serde(rename = "placement")]
    Placement(Placement),
    #[serde(rename = "spline")]
    Spline(Spline),
}

#[derive(Serialize, Deserialize)]
pub struct Sequence {
    objects: Vec<Object>,
}

impl Rendable for Sequence {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        for object in self.objects.iter() {
            match object {
                Object::Circle(element) => element.render(&context, querable),
                Object::Icon(element) => element.render(&context, querable),
                Object::Line(element) => element.render(&context, querable),
                Object::Placement(element) => element.render(&context, querable),
                Object::Sequence(element) => element.render(&context, querable),
                Object::Spline(element) => element.render(&context, querable),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Rendable for Line {
    fn render(&self, context: &Context, _: &Querable) {
        // recover proper line size
        let (_, y0) = context.device_to_user_distance(0.0, 0.0);
        let (_, y1) = context.device_to_user_distance(0.0, 1.0);
        context.set_line_width(y1 - y0);

        // draw line
        context.move_to(self.a.x, self.a.y);
        context.line_to(self.b.x, self.b.y);
        context.stroke();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spline {
    pub a: Point,
    pub b: Point,
    pub sa: Point,
    pub sb: Point,
}

impl Rendable for Spline {
    fn render(&self, context: &Context, _: &Querable) {
        // recover proper line size
        let (_, y0) = context.device_to_user_distance(0.0, 0.0);
        let (_, y1) = context.device_to_user_distance(0.0, 1.0);
        context.set_line_width(y1 - y0);

        // draw line
        context.move_to(self.a.x, self.a.y);
        context.curve_to(
            self.sa.x, self.sa.y, self.sb.x, self.sb.y, self.b.x, self.b.y,
        );
        context.stroke();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Circle {
    pub radius: f64,
}

impl Rendable for Circle {
    fn render(&self, context: &Context, querable: &Querable) {
        context.save();
        let palette = querable.palette();
        context.set_source_rgb(
            f64::from(palette.background_color.red),
            f64::from(palette.background_color.green),
            f64::from(palette.background_color.blue),
        );
        context.arc(0.0, 0.0, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        context.fill();
        context.restore();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    path: Vec<Vec<f64>>,
}

impl Rendable for Icon {
    fn render(&self, context: &Context, _: &Querable) {
        let mut first = true;
        for path in self.path.iter() {
            if first {
                context.move_to(path[0], path[1]);
                first = false;
            } else {
                Icon::draw_path_element(&path, &context);
            }
        }
        context.close_path();
        context.fill();
    }
}

impl Icon {
    // use proper function
    #[inline]
    fn draw_path_element(element: &Vec<f64>, context: &Context) {
        if element.len() == 2 {
            context.line_to(element[0], element[1]);
        } else if element.len() == 6 {
            context.curve_to(
                element[0], element[1], element[2], element[3], element[4], element[5],
            );
        }
        // todo
        // else return error
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
    size: Option<f64>,
    pub query: Query,
}

impl Placement {
    pub fn size(&self) -> f64 {
        match self.size {
            Some(size) => size,
            None => 100.0,
        }
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
        context.scale(0.01 * self.size(), 0.01 * self.size());

        let rendable = querable.get_element_from_query(&self.query);
        if rendable.is_some() {
            rendable.unwrap().render(&context, querable);
        }

        context.restore();
    }
}

#[derive(Serialize, Deserialize)]
pub enum Query {
    // search for a specific object
    #[serde(rename = "by_name")]
    ByName(String),
}
