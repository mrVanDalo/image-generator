extern crate cairo;

use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

use std::f64::consts::PI;
use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct Icon {
    path: Vec<Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
struct Structure {
    icons: HashMap<String, Icon>,
}

fn parse_example() {
    let mut file = File::open("./sketch/structure.json").expect("Couldn't create structure.json");
    let reader = BufReader::new(file);
    let structure: Structure = serde_json::from_reader(reader).unwrap();
    for key in structure.icons.keys() {
        println!("{}", key);
    }
}

fn main() {
    //render_example();
    parse_example();
}

// draw a path
// -----------
// context.move_to(x,y)
// context.line_to(x,y)
// context.curve_to(x0,y0,x1,y1,x, y); // curve to (x,y) with (x0,y0) as outgoing and (x1,y1) incoming curve
// context.close_path();

fn render_example() {
    let surface = ImageSurface::create(Format::Rgb24, 100, 100).expect("Can't create surface");
    let context = Context::new(&surface);

    // Examples are in 1.0 x 1.0 coordinate space
    // context.scale(120.0, 120.0);

    // Drawing code goes here
    context.set_line_width(1.0);
    context.set_source_rgb(1.0, 1.0, 1.0);

    context.rectangle(25.0, 25.0, 50.0, 50.0);
    context.stroke();

    context.arc(50.0, 50.0, 25.0, 0.0, 2.0 * PI);
    context.stroke();

    let mut file = File::create("file.png").expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
