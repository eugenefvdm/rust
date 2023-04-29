extern crate cairo;

use cairo::{Context, Format, ImageSurface};
use std::fs::File;

fn main() {
    println!("Hello, world!");

    let width = 300;
    let height = 200;

    let surface = ImageSurface::create(Format::ARgb32, width, height).unwrap();
    let context = Context::new(&surface);

    context.set_line_width(10.0);
    context.move_to(0.0, height as f64 / 2.0);
    context.line_to(width as f64, height as f64 / 2.0);
    context.stroke();

    let mut file = File::create("line.png").unwrap();
    surface.write_to_png(&mut file).unwrap();

    println!("A file line.png has been created.");
}
