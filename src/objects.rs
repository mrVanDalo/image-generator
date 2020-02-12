use crate::rendable::Rendable;
use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "icon")]
    Icon(Icon),
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
    fn render(&self, context: &Context) {
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
    fn render(&self, context: &Context) {
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
