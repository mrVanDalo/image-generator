use cairo::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Icon {
    path: Vec<Vec<f64>>,
}

impl Icon {
    pub fn render(&self, context: &Context) {
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
