use std::env;
use std::process::Command;

// Define an enum to represent the possible commands
enum Cmd {
    Ping(String),
    Hostname,
}

fn main() {
    // Get the command and argument from the command line arguments
    let args: Vec<String> = env::args().collect();
    let command_str = &args[1];

    // Parse the command and argument using a match statement
    let command = match command_str.as_str() {
        "ping" => Cmd::Ping(args[2].clone()),
        "hostname" => Cmd::Hostname,
        _ => {
            // If the command is not recognized, output an error message
            println!("Unrecognized command: {}", command_str);
            return;
        }
    };

    // Execute the appropriate command based on the parsed command
    match command {
        Cmd::Ping(ip_address) => ping(ip_address),
        Cmd::Hostname => hostname(),
    }
}

// Define a function to execute the hostname command
fn hostname() {
    // Get the hostname using the hostname command
    let output = Command::new("hostname")
            .arg("--fqdn")
            .output()
            .expect("failed to execute process");
    let hostname = String::from_utf8_lossy(&output.stdout);

    // Output the hostname
    println!("{}", hostname.trim_end());
}

// Define a function to execute the ping command with the given IP address
fn ping(ip_address: String) {
    loop {
        // Run the ping command once and capture the avg output
        let output = Command::new("ping")
            .arg("-c")
            .arg("2")
            .arg(&ip_address)
            .output()
            .expect("failed to execute process");

        // Parse the output to extract the average time in milliseconds
        let output_str = String::from_utf8_lossy(&output.stdout);
        let time_index = output_str.find("rtt min/avg/max/mdev = ").unwrap() + 24;
        let time_str = &output_str[time_index + 5..time_index + 10];
        let time_ms = time_str.parse::<f32>().unwrap();

        // Output the average time in milliseconds
        // println!("Average ping time for {}: {} ms", ip_address, time_str);
        println!("Average ping time for {}: {} ms", ip_address, time_ms);
    }
}
