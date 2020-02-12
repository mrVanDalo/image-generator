use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

//use std::f64::consts::PI;
use std::fs::File;

mod composition;
mod objects;
mod palette;
mod rendable;
mod structure;
mod tag;

use crate::objects::Object;
use crate::palette::Palette;
use crate::rendable::Rendable;
use crate::structure::Structure;

fn main() {
    let structure = Structure::load_from_file("./sketch/example.json").unwrap();
    let surface = ImageSurface::create(
        Format::Rgb24,
        structure.get_image_width(),
        structure.get_image_height(),
    )
    .expect("Can't create surface");
    let context = Context::new(&surface);

    let palette = Palette::dark_on_bright(Palette::random_color());

    // set background color
    context.set_source_rgb(
        f64::from(palette.background_color.red),
        f64::from(palette.background_color.green),
        f64::from(palette.background_color.blue),
    );
    context.rectangle(
        0.,
        0.,
        f64::from(structure.get_image_width()),
        f64::from(structure.get_image_height()),
    );
    context.fill();

    // configure line
    context.set_source_rgb(
        f64::from(palette.fill_color.red),
        f64::from(palette.fill_color.green),
        f64::from(palette.fill_color.blue),
    );
    context.set_line_width(1.0);

    structure.render(&context);

    render_image("file.png", &surface);
}

fn render_image(path: &str, surface: &ImageSurface) {
    let mut file = File::create(&path).expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
