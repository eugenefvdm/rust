extern crate cairo;

use cairo::{Context, Format, ImageSurface}; // Import the Cairo library for line drawing on a surface
use std::fs::File; // Import the File library for writing to a file
use std::io; // Import the IO library for reading and writing to the console


/*  
    Main entry point into the application.

    This is a demo application to showcase various Rust features.

    - Print Hello World
    - Create a line on a surface and save it to a file using the Cairo library
    - Draw some text characters using SINE and COSINE functions
    - Create a menu and handle keyboard input
*/
fn main() {
    println!("Hello, world!");

    line();

    wait();

    sine_cosine();

    println!("Above is the SINE/COSINE graph.");

    println!("Next there will be a menu.");

    wait();

    menu();
    
}

/* 
    Sine Consine example    

    Constants for the width and height of the canvas.

    These are used to create a 2D array of characters to represent the canvas.
    The canvas is then printed to the console.
*/
const WIDTH: i32 = 100;
const HEIGHT: i32 = 50;

fn sine_cosine() {
    let mut canvas = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];

    for x in 0..WIDTH {
        let y = (HEIGHT as f64 / 2.0 - (HEIGHT as f64 / 2.0 * (x as f64 / WIDTH as f64).sin())) as i32;
        canvas[y as usize][x as usize] = '*';
    }

    for y in 0..HEIGHT {
        let x = (WIDTH as f64 / 2.0 + (HEIGHT as f64 / 2.0 * (y as f64 / HEIGHT as f64).cos())) as i32;
        canvas[y as usize][x as usize] = '*';
    }

    for row in canvas {
        println!("{}", row.into_iter().collect::<String>());
    }
}

fn line() {
    let surface_width = 300;
    let surface_height = 200;

    let surface = ImageSurface::create(Format::ARgb32, surface_width, surface_height).unwrap();
    let context = Context::new(&surface);

    context.set_line_width(10.0);
    context.move_to(0.0, surface_height as f64 / 2.0);
    context.line_to(surface_width as f64, surface_height as f64 / 2.0);
    context.stroke();

    let mut file = File::create("line.png").unwrap();
    surface.write_to_png(&mut file).unwrap();

    println!("A file line.png has been created.");
}

fn menu() {
    let menu_items = vec!["Option 1", "Option 2", "Option 3", "Quit"];
    let mut selected_item = 0;

    loop {
        // Clear the console
        print!("\x1B[2J\x1B[1;1H");

        // Display the menu
        for (i, item) in menu_items.iter().enumerate() {
            if i == selected_item {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        // Get keyboard input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Handle keyboard input
        match input.trim() {
            "q" => break,
            "\n" => {
                // Execute the selected menu item
                match selected_item {
                    0 => println!("Option 1 selected"),
                    1 => println!("Option 2 selected"),
                    2 => println!("Option 3 selected"),
                    _ => break,
                }
            },
            "\x1B[A" => {
                // Move up the menu
                if selected_item > 0 {
                    selected_item -= 1;
                }
            },
            "\x1B[B" => {
                // Move down the menu
                if selected_item < menu_items.len() - 1 {
                    selected_item += 1;
                }
            },
            _ => {}
        }
    }
}

fn wait() {
    println!("Press enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}