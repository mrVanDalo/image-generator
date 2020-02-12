use crate::composition::Composition;
use crate::rendable::Rendable;
use crate::structure::Querable;
use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "icon")]
    Icon(Icon),
    #[serde(rename = "sequence")]
    Sequence(Sequence),
    #[serde(rename = "composition")]
    Composition(Composition),
}

#[derive(Serialize, Deserialize)]
pub struct Sequence {
    objects: Vec<Object>,
}

impl Rendable for Sequence {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        for object in self.objects.iter() {
            match object {
                Object::Line(element) => element.render(&context, querable),
                Object::Icon(element) => element.render(&context, querable),
                Object::Sequence(element) => element.render(&context, querable),
                Object::Composition(element) => element.render(&context, querable),
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
