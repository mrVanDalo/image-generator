use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

//use std::f64::consts::PI;
use std::fs::File;

mod composition;
mod icon;
mod structure;

use structure::Structure;

fn main() {
    let structure = Structure::load_from_file("./sketch/structure.json").unwrap();
    let surface = ImageSurface::create(
        Format::Rgb24,
        structure.get_image_width(),
        structure.get_image_height(),
    )
    .expect("Can't create surface");
    let context = Context::new(&surface);

    // set background color
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.rectangle(
        0.,
        0.,
        f64::from(structure.get_image_width()),
        f64::from(structure.get_image_height()),
    );
    context.fill();

    // configure line
    context.set_source_rgb(0., 0., 0.);

    // composition sketch
    for composition in &structure.compositions {
        context.save();
        context.translate(composition.x, composition.y);
        context.scale(0.01 * composition.size(), 0.01 * composition.size());

        let rendable = structure.get_element_from_query(&composition.query);
        if rendable.is_some() {
            rendable.unwrap().render(&context);
        }

        context.restore();
    }

    render_image("file.png", &surface);
}

fn render_image(path: &str, surface: &ImageSurface) {
    let mut file = File::create(&path).expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}

// draw a path
// -----------
// context.move_to(x,y)
// context.line_to(x,y)
// context.curve_to(x0,y0,x1,y1,x, y); // curve to (x,y) with (x0,y0) as outgoing and (x1,y1) incoming curve
// context.close_path();
