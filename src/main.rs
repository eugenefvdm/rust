extern crate cairo;

use cairo::{Context, Format, ImageSurface}; // Import the Cairo library for line drawing on a surface
use fs2::{free_space, total_space}; // Disk space utilities
use mysql::Pool;
use mysql::prelude::*;
use std::fs::File; // Writing to a file
use std::io; // Reading and writing to the console
use std::io::Write; // Writing to a file
use std::fs; // Read a file
use std::fs::OpenOptions; // File search and replace
use std::io::{BufRead, BufReader}; // File search and replace
use std::path::Path;
use std::process::Command;
use regex::Regex;
use prettytable::{Table, Row, Cell};
use prettytable::format;
use std::thread;
use std::time::Duration; // Also used by threads
use math::convert_bytes;
use security::{encrypt, decrypt};
use std::str::from_utf8;

fn directory_listing() {
    let output = Command::new("ls")
                    .arg("-lah")
                     .output()
                     .expect("Failed to execute command");

    println!("Directory listing Output:\n{}", String::from_utf8_lossy(&output.stdout));
}

fn disk_space()  {    
    let path = "/";
    
    let total_space = total_space(path).unwrap();
    let free_space = free_space(path).unwrap();
    let used_space = total_space - free_space;

    println!("SIZE:      {} bytes", convert_bytes(total_space));
    println!("USED:      {} bytes", convert_bytes(used_space));    
    println!("AVAILABLE: {} bytes", convert_bytes(free_space));    
}

fn draw_line_on_surface() {
    let surface_width = 300;
    let surface_height = 200;

    let surface = ImageSurface::create(Format::ARgb32, surface_width, surface_height).unwrap();
    let context = Context::new(&surface);

    context.set_line_width(10.0);
    context.move_to(0.0, surface_height as f64 / 2.0);
    context.line_to(surface_width as f64, surface_height as f64 / 2.0);
    context.stroke();

    let mut file = File::create("assets/line_on_surface.png").unwrap();
    surface.write_to_png(&mut file).unwrap();

    println!("A file assets/line_on_surface.png has been created.");
}

/*
    Good example of a recursive function that calls itself
 */
fn directory_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            size += entry.metadata()?.len();
        } else {
            size += directory_size(&entry_path)?;
        }
    }

    Ok(size)
}

fn linux_service_restart() {
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

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name: Option<String>,
    email: Option<String>,
}

fn mysql_read_users_table() -> std::result::Result<(), Box<dyn std::error::Error>> {    
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

    println!("Yay! Found {:?} MySQL users:", selected_users.len());

    for user in selected_users {
        println!(" {:?}", user);
    }

    Ok(())
    
}

fn print_file_contents(filename: &str) -> io::Result<()> {
    let content = fs::read_to_string(filename)?;

    println!("{}", content);

    Ok(())
}

fn read_disk_space(path: &str) -> std::io::Result<u64> {
    let path = Path::new(path);

    let size = directory_size(path).unwrap();

    //println!("Size of {:?}: {} bytes", path, size);

    Ok(size)
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

fn encrypt_decrypt() {
    let data = "Hello, world!";
    let password = "12345";

    println!("Data to encrypt: \"{}\" and password: \"{}\"", &data, &password);

    // println!("Encrypting now");
    let res = encrypt(data.as_bytes(), password);
    println!("Encrypted response: {}", res);

    // println!("Decrypting the response");
    let decrypted_bytes = decrypt(res.as_str(), password).unwrap();
    let decrypted_string = from_utf8(&decrypted_bytes).unwrap();
    println!("Decrypted response: {}", decrypted_string);
}

// Start of encryption / decryption functions
// Original example from https://www.boringadv.com/2022/12/05/simple-encryption-in-rust/#comment-83
// Refactored using ChatGPT

///encrypt "data" using "password" as the password
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)
// pub fn encrypt(data: &[u8], password: &str) -> String {
//     let key_size = crypto::aes::KeySize::KeySize128;

//     //pad or truncate the key if necessary
//     let valid_key = get_valid_key(password);
//     let iv = get_iv(12); //initial vector (iv), also called a nonce
//     let mut cipher = AesGcm::new(key_size, &valid_key, &iv, &[]);

//     //create a vec of data.len 0's. This is where the encrypted data will be saved.
//     //the encryption is performed in-place, so this vector of 0's will be converted
//     //to the encrypted data
//     let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();

//     //create a vec of 16 0's. This is for the mac. This library calls it a "tag", but it's really
//     // the mac address. This vector will be modified in place, just like the "encrypted" vector
//     // above
//     let mut mac: Vec<u8> = repeat(0).take(16).collect();

//     //encrypt data, put it into "encrypted"
//     cipher.encrypt(data, &mut encrypted, &mut mac[..]);

//     //create the output string that contains the nonce, cipher text, and mac
//     let hex_iv = hex::encode(iv);
//     let hex_cipher = hex::encode(encrypted);
//     let hex_mac = hex::encode(mac);
//     let output = format!("{}/{}/{}", hex_iv, hex_cipher, hex_mac);

//     output
// }

// fn get_valid_key(key: &str) -> Vec<u8> {
//     let mut bytes = key.as_bytes().to_vec();
//     bytes.resize(16, 0x00);

//     bytes
// }

// fn get_iv(size: usize) -> Vec<u8> {
//     let mut iv = vec![0u8; size];
//     rand::thread_rng().fill_bytes(&mut iv);

//     iv
// }

// /// Decryption using AES-GCM 128
// /// iv_data_mac is a string that contains the iv/nonce, data, and mac values. All these values
// /// must be hex encoded, and separated by "/" i.e. [hex(iv)/hex(data)/hex(mac)]. This function decodes
// /// the values. key (or password) is the raw (not hex encoded) password
// pub fn decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
//     let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
//     let key = get_valid_key(key);

//     let key_size = crypto::aes::KeySize::KeySize128;

//     // I don't use the aad for verification. aad isn't encrypted anyway, so it's just specified
//     // as &[].
//     let mut decipher = AesGcm::new(key_size, &key, &iv, &[]);

//     // create a list where the decoded data will be saved. dst is transformed in place. It must be exactly the same
//     // size as the encrypted data
//     let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
//     let _result = decipher.decrypt(&data, &mut dst, &mac);

//     // if result { println!("Successful decryption"); }
//     // println!("\nDecrypted {}", str::from_utf8(&dst).unwrap());

//     Ok(dst)
// }

// /// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
// /// is the data returned from encrypt(). This function splits the data, removes
// /// the hex encoding, and returns each as a list of bytes.
// fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
//     let split: Vec<&str> = orig.split('/').collect();

//     let iv = decode_hex(split[0])?;
//     let data = decode_hex(split[1])?;
//     let mac = decode_hex(split[2])?;

//     Ok((iv, data, mac))
// }

// fn decode_hex(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
//     hex::decode(hex_str).map_err(|_| Box::new(io::Error::from(ErrorKind::Other)) as Box<dyn Error>)
// }

// End of encryption / decryption functions

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

// Using prettiertable
fn table_formatting() {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("FwB"),
        Cell::new("Name").style_spec("FwB"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("1"),
        Cell::new("Joe"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("2"),
        Cell::new("Pete"),
    ]));

    table.printstd();   
}

fn text_file_creation() {
    let mut file = File::create("assets/vhost.example.conf").unwrap();

    let indentation = "    "; // 4 spaces

    write!(file, "<VirtualHost 129.232.252.163:80>\n").unwrap();
    write!(file, "{}SuexecUserGroup \"#1356\" \"#1056\"\n", indentation).unwrap();
    write!(file, "{}ServerName example.com\n", indentation).unwrap();
    write!(file, "{}ServerAlias www.example.com\n", indentation).unwrap();
    write!(file, "{}ServerAlias webmail.example.com\n", indentation).unwrap();  
}

/*
    Basic thread example that includes handle.join() to wait for the thread to finish.
 */
fn threads() {    
    let handle= thread::spawn(|| {
        for i in 1..4 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}


fn wait() {
    println!("\nPress enter to continue..");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

/*  
    Main entry point to the application.
*/
fn main() -> io::Result<()>{
    println!("Hello, world!\n");

    encrypt_decrypt();
    wait();
    
    threads();
    wait();

    table_formatting();
    wait();

    disk_space();
    println!("^^^ This is your disk size ^^^");    
    wait();

    let result = mysql_read_users_table();
    assert!(result.is_ok());

    wait();

    linux_service_status();

    wait();

    directory_listing();
    println!("^^^ Here is `ls -la` output ^^^\n");
        
    println!("Next we're going to output the size of a directory:");  
    let path = "assets";
    let result = read_disk_space(path);    
    println!("Size of {:?} is {} bytes", path, result.unwrap());    
    wait();    
    
    text_file_creation();
            
    let search = "129.232.252.163";
    let replace = "1.1.1.1";
    let filename = "assets/vhost.example.conf";
    replace_file_content(search, replace, filename);
    print_file_contents(filename)?;
    println!("^^^ A file was created and then search and replace took place. ^^^\n");

    draw_line_on_surface();    
    wait();

    sine_cosine();    
    println!("Above is the sine and consine graph.\n");

    println!("Next we're going to restart nginx. You can press escape to cancel.");
    wait();

    linux_service_restart();

    println!("\nNext there will be a menu.");
    wait();    
    menu();

    Ok(()) // Return value required by main function because it expects a io::Result<()>
    
}