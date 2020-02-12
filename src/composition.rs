use crate::rendable::Rendable;
use crate::structure::Querable;
use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Composition {
    pub placement: Placement,
    size: Option<f64>,
    pub query: Query,
}

#[derive(Serialize, Deserialize)]
pub enum Placement {
    #[serde(rename = "absolute")]
    Absolute {
        x: f64,
        y: f64,
        #[serde(default)]
        angle: f64,
    },
    #[serde(rename = "relative")]
    Relative {
        x: f64,
        y: f64,

        #[serde(default)]
        angle: f64,
    },
}

impl Composition {
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

impl Rendable for Composition {
    fn render(&self, context: &Context, querable: &dyn Querable) {
        context.save();

        match self.placement {
            Placement::Absolute { x, y, angle } => {
                context.translate(x, y);
                context.rotate(degree_to_radian(angle));
            }
            Placement::Relative { x, y, angle } => {
                //context.translate(
                //    x / 100.0 * f64::from(self.get_image_width()),
                //    y / 100.0 * f64::from(self.get_image_height()),
                //);
                context.rotate(degree_to_radian(angle));
            }
        }

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
