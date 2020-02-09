extern crate cairo;

use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

use std::fs::File;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 120, 120).expect("Can't create surface");
    let context = Context::new(&surface);
    // Examples are in 1.0 x 1.0 coordinate space
    context.scale(120.0, 120.0);

    // Drawing code goes here
    context.set_line_width(0.1);
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.rectangle(0.25, 0.25, 0.5, 0.5);
    context.stroke();

    let mut file = File::create("file.png").expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
