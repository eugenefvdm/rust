extern crate cairo;

use cairo::{Context, Format, ImageSurface}; // Import the Cairo library for line drawing on a surface
use std::fs::File; // Import the File library for writing to a file
use std::io; // Import the IO library for reading and writing to the console
use std::io::Write; // For writing to a file
use std::fs; // Used to read a file
use std::fs::{OpenOptions}; // Used by file search and replace
use std::io::{BufRead, BufReader}; // Also used by file search and replace
use fs2::{free_space, total_space}; // Used to get disk space
use std::path::Path; // Used to get disk space
use std::process::Command;
use regex::Regex; // Used for regular expressions
use mysql::{Pool};
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name: Option<String>,
    email: Option<String>,
}

fn read_mysql_table() -> std::result::Result<(), Box<dyn std::error::Error>> {    
    let url = "mysql://root:@localhost:3306/blog";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_users = conn
        .query_map(
            "SELECT id, name, email FROM users",
            |(id, name, email)| {
                User { id, name, email }
            },
        )?;

    println!("Yay!");

    println!("Found {:?} users", selected_users.len());

    for user in selected_users {
        println!("Found user {:?}", user);
    }

    Ok(())
    
}

fn linux_service_status() {
    let output = Command::new("systemctl")
                    .arg("status")
                    .arg("nginx")
                     .output()
                     .expect("Failed to execute command");

    let text_output = String::from_utf8_lossy(&output.stdout);

    let re = Regex::new(r"Active: active").unwrap();
    
    if re.is_match(&text_output) {
        println!("Nginx is running");
    } else {
        println!("Nginx is not running");
    }    
}

fn get_directory_listing() {
    let output = Command::new("ls")
                    .arg("-lah")
                     .output()
                     .expect("Failed to execute command");

    println!("Output {}", String::from_utf8_lossy(&output.stdout));
}

fn restart_linux_service() {
    let service_name = "nginx"; // Replace with your service name

    let mut systemctl = Command::new("systemctl");
    
    systemctl.arg("restart").arg(service_name);

    let output = match systemctl.output() {
        Ok(output) => output,
        Err(e) => {
            println!("Error occurred: {}", e);            
            return;
        }
    };

    if output.status.success() {
        println!("Success {}", output.status);        
    } else {
        println!("Failed {}", output.status);        
    }
}

fn read_disk_space() {
    let path = Path::new("/home/eugene/code/hello_world_rust/assets");

    let size = get_directory_size(path).unwrap();

    println!("Size of {:?}: {} bytes", path, size);

    wait();
}

/*
    Good example of an recursive function that calls itself
 */
fn get_directory_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            size += entry.metadata()?.len();
        } else {
            size += get_directory_size(&entry_path)?;
        }
    }

    Ok(size)
}

fn get_total_disk_size()  {    
    let path = "/home/eugene/code/hello_world_rust";
    
    let total_space = total_space(path).unwrap();
    let free_space = free_space(path).unwrap();
    let used_space = total_space - free_space;

    println!("Total space: {} bytes", total_space);
    println!("Free space: {} bytes", free_space);
    println!("Used space: {} bytes", used_space);
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
    let mut file = File::create("vhost.example.conf").unwrap();

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

/*  
    Main entry point into the application.
*/
fn main() -> io::Result<()>{
    println!("Hello, world!");

    let result = read_mysql_table();
    assert!(result.is_ok());

    wait();

    linux_service_status();

    wait();

    get_directory_listing();

    restart_linux_service();

    read_disk_space();

    get_total_disk_size();

    write_text_file();

    print_file_contents("vhost.example.conf")?;
    
    wait();

    let search = "129.232.252.163";
    let replace = "1.1.1.1";
    let filename = "vhost.example.conf";

    replace_file_content(search, replace, filename);

    print_file_contents("vhost.example.conf")?;

    line();

    wait();

    sine_cosine();

    println!("Above is the SINE/COSINE graph.");

    println!("Next there will be a menu.");

    wait();

    menu();

    Ok(()) // Return value required by main function
    
}