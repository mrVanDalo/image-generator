//!A tool to create random background images.

use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

use std::fs::File;

pub mod objects;
pub mod palette;
pub mod rendable;
pub mod structure;
pub mod tag;

use crate::rendable::Rendable;
use crate::structure::ImageContext;
use crate::structure::Structure;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    /// Output file (will be a png)
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Input file (in json)
    #[structopt(name = "input.json", parse(from_os_str))]
    input: PathBuf,

    /// Optional : override width (default is 100)
    /// you can also set width in the input.json
    #[structopt(long)]
    width: Option<i32>,

    /// Optional : override height (default is 100)
    /// you can also set height in the input.json
    #[structopt(long)]
    height: Option<i32>,

    /// Optional : override depth (default is 30)
    /// of recursion allowed.
    #[structopt(long)]
    depth: Option<i32>,

    /// Optional : override line size (default is 1.0)
    /// you can also set line_size in the input.json
    #[structopt(long)]
    line_size: Option<f64>,
}

fn main() {
    // parse options
    let opt = Opt::from_args();

    if !opt.input.exists() {
        println!("{}, does not exist", opt.input.to_str().unwrap());
        std::process::exit(1);
    }
    let structure = Structure::load_from_file(&opt.input.to_str().unwrap()).unwrap();
    let image_context = ImageContext::new(&structure);

    let width = match opt.width {
        Some(width) => width,
        None => structure.width,
    };
    let height = match opt.height {
        Some(height) => height,
        None => structure.height,
    };
    let line_size = match opt.line_size {
        Some(line_size) => line_size,
        None => structure.line_size,
    };
    let depth = match opt.depth {
        Some(depth) => depth,
        None => structure.depth,
    };

    let surface = ImageSurface::create(Format::Rgb24, width, height).expect("Can't create surface");
    let context = Context::new(&surface);
    let palette = image_context.palette();

    // set background color
    context.set_source_rgb(
        f64::from(palette.background_color.red),
        f64::from(palette.background_color.green),
        f64::from(palette.background_color.blue),
    );
    context.rectangle(0., 0., f64::from(width), f64::from(height));
    context.fill();

    // configure line
    context.set_source_rgb(
        f64::from(palette.fill_color.red),
        f64::from(palette.fill_color.green),
        f64::from(palette.fill_color.blue),
    );
    context.set_line_width(line_size);

    // center
    context.translate(f64::from(width) / 2.0, f64::from(height) / 2.0);
    structure.render(&context, &image_context, depth);

    render_image(&opt.output.to_string_lossy(), &surface);
}

fn render_image(path: &str, surface: &ImageSurface) {
    let mut file = File::create(&path).expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("{}, created", path),
        Err(_) => println!("Error create file.png"),
    }
}
