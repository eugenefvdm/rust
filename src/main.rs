extern crate cairo;

use cairo::{Context, Format, ImageSurface}; // Import the Cairo library for line drawing on a surface
use std::fs::File; // Import the File library for writing to a file
use std::io; // Import the IO library for reading and writing to the console
use std::io::Write; // For writing to a file
use std::fs; // Used to read a file
use std::fs::{OpenOptions}; // Used by file search and replace
use std::io::{BufRead, BufReader}; // Also used by file search and replace

/*  
    Main entry point into the application.
*/
fn main() -> io::Result<()>{
    println!("Hello, world!");

    write_text_file();

    print_file_contents("vhost.conf")?;
    
    wait();

    let search = "129.232.252.163";
    let replace = "1.1.1.1";
    let filename = "vhost.conf";

    replace_file_content(search, replace, filename);

    print_file_contents("vhost.conf")?;

    line();

    wait();

    sine_cosine();

    println!("Above is the SINE/COSINE graph.");

    println!("Next there will be a menu.");

    wait();

    menu();

    Ok(()) // Return value required by main function
    
}

fn replace_file_content(search: &str, replace: &str, filename: &str)  {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut new_content = String::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let new_line = line.replace(search, replace);
        new_content.push_str(&new_line);
        new_content.push('\n');
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .expect("Could not open file for writing");

    file.write_all(new_content.as_bytes())
        .expect("Could not write to file");
}

fn write_text_file() {
    let mut file = File::create("vhost.conf").unwrap();

    let indentation = "    "; // 4 spaces

    write!(file, "<VirtualHost 129.232.252.163:80>\n").unwrap();
    write!(file, "{}SuexecUserGroup \"#1356\" \"#1056\"\n", indentation).unwrap();
    write!(file, "{}ServerName example.com\n", indentation).unwrap();
    write!(file, "{}ServerAlias www.example.com\n", indentation).unwrap();
    write!(file, "{}ServerAlias webmail.example.com\n", indentation).unwrap();  
}

fn print_file_contents(filename: &str) -> io::Result<()> {
    let content = fs::read_to_string(filename)?;

    println!("{}", content);

    Ok(())
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

// Sine Consine example variables    
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



fn menu() {
    let menu_items = vec!["Option 1", "Option 2", "Option 3", "Quit (q)"];
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