use std::env;
use std::process::Command;
use regex::Regex;

// Define an enum to represent the possible commands
enum Cmd {
    Fail2ban(String, String),
    Hostname,
    Ping(String),
    SearchEmail(String, String),    
}

fn main() {
    // Get the command and argument from the command line arguments
    let args: Vec<String> = env::args().collect();
    let command_str = &args[1];

    // Parse the command and argument using a match statement
    let command = match command_str.as_str() {
        "fail2ban" => Cmd::Fail2ban(args[2].clone(), args[3].clone()),
        "hostname" => Cmd::Hostname,
        "ping" => Cmd::Ping(args[2].clone()),
        "search_email" => Cmd::SearchEmail(args[2].clone(), args[3].clone()),        
        _ => {
            // If the command is not recognized, output an error message
            println!("Unrecognized command: {}", command_str);
            println!("List of commands:");
            println!(" fail2ban <server> <ip_address>");
            println!(" hostname");
            println!(" ping <ip_address>");
            println!(" search_email <server> <email>");
            return;
        }
    };

    // Execute the appropriate command based on the parsed command
    match command {
        Cmd::Fail2ban(server, ip_address) => fail2ban(server, ip_address),
        Cmd::Hostname => hostname(),
        Cmd::Ping(ip_address) => ping(ip_address),
        Cmd::SearchEmail(server, email) => search_email(server, email),        
    }
}

fn fail2ban(server: String, ip_address: String) {       
    // Set up the SSH command
    let ssh_command = format!("ssh root@{} -p22222 'grep \"\\] Ban {}\" /var/log/fail2ban.log'", server, ip_address);

    println!("SSH command: {}", ssh_command);

    // Run the SSH command and capture the output
    let output = Command::new("bash")
                         .arg("-c")
                         .arg(&ssh_command)
                         .output()
                         .expect("Failed to execute SSH command");

    // Convert the output to a string
    let output_string = String::from_utf8(output.stdout).unwrap();

    // Search for the pattern in the output
    if let Some(index) = output_string.find(&format!("] Ban {}", ip_address)) {
        // Extract the time from the log file
        let time = &output_string[0..19];
        println!("Time of ban: {} #{}", time, index);
    } else {
        println!("Pattern not found");
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

fn search_email(server: String, email: String) {           
    let output_string = search_email_ssh_command(server, &email);
    
    // If debug
    //println!("Here is the output string:\n\n{}", output_string);
        
    // From / To Events, output with recipient first so that's there's better formatting
    let re = Regex::new(r"^([A-Za-z]{3}\s+\d{1,2}\s\d{2}:\d{2}:\d{2}).+from=(.+); receiver=(.+)").unwrap();    
    let mut total = 0;    
    for line in output_string.lines() {
        if let Some(capture) = re.captures(line) {
            println!("{} {} {}", &capture[1], &capture[3], &capture[2]);
            total += 1;
        }
    }
    println!("There were {} from/to pass events.", total);

    // Actual local deliveries
    let re = Regex::new(&format!(r"^([A-Za-z]{{3}}\s+\d{{1,2}}\s\d{{2}}:\d{{2}}:\d{{2}}).+orig_to=<({})>.+(status=sent)", email)).unwrap();
    let mut total = 0;    
    for line in output_string.lines() {
        if let Some(capture) = re.captures(line) {
            println!("{} {} {}", &capture[1], &capture[2], &capture[3]);
            total += 1;
        }
    }
    println!("There were {} actual local deliveries.", total);

}

fn search_email_ssh_command(server: String, email: &String) -> String {
    // Set up the global SSH command
    let ssh_command = format!("ssh root@{} -p22222 'egrep -i \"Pass.+mailfrom.+envelope-from.+{}|orig_to=<{}>\" /var/log/maillog'", server, email, email);

    // If debug
    // println!("Global egrep regex to get all events:\n{}", ssh_command);

    // Run the SSH command and capture the output
    let output = Command::new("bash")
                         .arg("-c")
                         .arg(&ssh_command)
                         .output()
                         .expect("Failed to execute SSH command");
    
    return String::from_utf8(output.stdout).unwrap().to_lowercase();
}