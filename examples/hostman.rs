use std::env;
use std::process::Command;
use regex::Regex;
use prettytable::{Table, Row, Cell};
use prettytable::format;
use sqlite::{Connection};

// Define an enum to represent the possible commands
enum Cmd {
    Fail2ban(String, String),
    History,
    Hostname,
    Ping(String),
    ProcessList(String, Option<u16>, Option<String>),
    SearchEmail(String, String),
    SearchEmailLog(String, String),
}

fn main() {
    // Get the command and argument from the command line arguments
    let args: Vec<String> = env::args().collect();
    let command_str = &args[1];

    // Parse the command and argument using a match statement
    let command = match command_str.as_str() {
        "fail2ban" => Cmd::Fail2ban(args[2].clone(), args[3].clone()),

        "history" => Cmd::History,

        "hostname" => Cmd::Hostname,

        "ping" => Cmd::Ping(args[2].clone()),

        "process_list" => {
            if args.len() > 4 {
                let port = args[3].parse::<u16>().ok();
                let username = args[4].parse::<String>().ok();

                Cmd::ProcessList(args[2].clone(), port, username)
            } else if args.len() > 3 {
                let port = args[3].parse::<u16>().ok();

                Cmd::ProcessList(args[2].clone(), port, None)
            } else {
                Cmd::ProcessList(args[2].clone(), None, None)
            }
        }
        
        "search_email" => Cmd::SearchEmail(args[2].clone(), args[3].clone()),        

        "search_email_log" => Cmd::SearchEmailLog(args[2].clone(), args[3].clone()),        

        _ => {
            // If the command is not recognized, output an error message
            println!("Unrecognized command: {}", command_str);
            println!("List of commands:");
            println!(" fail2ban <host> <ip_address>");
            println!(" history");
            println!(" hostname");
            println!(" process_list <host> [port] [username]");
            println!(" ping <ip_address>");
            println!(" search_email <host> <email>");
            println!(" search_email_log <host> <ip_address>");
            return;
        }
    };

    // Execute the appropriate command based on the parsed command
    match command {
        Cmd::Fail2ban(server, ip_address) => fail2ban(server, ip_address),
        Cmd::History => history(),
        Cmd::Hostname => hostname(),
        Cmd::ProcessList(server,port, username) => process_list(server, port, username),
        Cmd::Ping(ip_address) => ping(ip_address),
        Cmd::SearchEmail(server, email) => search_email(server, email),        
        Cmd::SearchEmailLog(server, ip_address) => search_email_log(server, ip_address),
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

    let result: String;
    
    // Search for the pattern in the output
    if let Some(_index) = output_string.find(&format!("] Ban {}", ip_address)) {
        // Extract the time from the log file characters 0 through 19
        let time = &output_string[0..19];
        result = format!("{} was banned {}", ip_address, time);
        println!("{}", result);
    } else {
        result = String::from("Pattern not found");
        println!("{}", result);
    }

    let command = format!("fail2ban=>{}=>{}", server, ip_address);
    
    store(command, result);

}

// Output the SQLite history table
// TODO having trouble converting the ID be accepted 
fn history() {
    // Connect to the database
    let connection = sqlite::open("history.db").unwrap();

    let query = "SELECT id, command, output, created_at, updated_at FROM history ORDER BY created_at ASC";
    
    // Create a new table
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    
    // Add headers to the table
    table.add_row(Row::new(vec![
        // Cell::new("ID").style_spec("FwB"),
        Cell::new("Command").style_spec("FwB"),
        Cell::new("Output").style_spec("FwB"),
        Cell::new("Created At").style_spec("FwB"),
        Cell::new("Updated At").style_spec("FwB"),
    ]));

    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()        
        .map(|row| row.unwrap())
        {
            table.add_row(Row::new(vec![
                // Cell::new(row.read::<&i64, _>("id")),
                Cell::new(row.read::<&str, _>("command")),
                Cell::new(row.read::<&str, _>("output")),
                Cell::new(row.read::<&str, _>("created_at")),
                Cell::new(row.read::<&str, _>("updated_at")),
            ]));            
        }

    // Print the table to the console
    table.printstd();

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

    // Return the hostname
    // hostname.trim_end().to_string()
}

// Get the number of processes running on a remote server
fn process_list(server: String, port: Option<u16>, username: Option<String>) {  
    // Construct the SSH command with optional port and username arguments
    let mut ssh_command = Command::new("ssh");
    
    if let Some(username) = username {
        ssh_command.arg(format!("{}@{}", username, server));
    } else {
        ssh_command.arg(format!("root@{}", server));        
    }
            
    if let Some(port) = port {
        ssh_command.arg("-p").arg(port.to_string());
    }

    ssh_command.arg("ps ax");

    // Use SSH to execute the ps command on the remote server
    let output = ssh_command.output().expect("failed to execute process");
    // Process the output and count the number of lines
    let stdout = String::from_utf8_lossy(&output.stdout);
    let num_processes = stdout.lines().count() - 1; // Exclude the header line
    let message = format!("{} processes running.", num_processes);

    println!("{}", message);

    let command = format!("ps ax on {:?}", server);

    store(command, message);
}

// Execute ping in a loop and output average time
fn ping(ip_address: String) {
    let mut count = 0;

    loop {
        // Run the ping command twice and capture the avg output
        let output = Command::new("ping")
            .arg("-c")
            .arg("2")
            .arg(&ip_address)
            .output()
            .expect("failed to execute process");

        // Parse the output to extract the average time in milliseconds
        let output_str = String::from_utf8_lossy(&output.stdout);
                        
        let re = Regex::new(r"rtt min/avg/max/mdev = [\d.]+/([\d.]+)/[\d.]+/[\d.]+ ms").unwrap();

        let average = re.captures(&output_str).unwrap().get(1).unwrap().as_str();

        println!("ping {}=>{} ms", ip_address, average);
        
        count += 1;

        if count == 1 {
            store(format!("ping=>{}", ip_address), format!("{} ms", average));
        }
    }
}

fn search_email(server: String, email: String) {           
    let output_string = ssh_command(&server, &email);
            
    // from= and receiver= events
    let re = Regex::new(r"^([A-Za-z]{3}\s+\d{1,2}\s\d{2}:\d{2}:\d{2}).+from=(.+); receiver=(.+)").unwrap();    

    let mut total = 0;    

    for line in output_string.lines() {
        if let Some(capture) = re.captures(line) {
            println!("{} {} {}", &capture[1], &capture[3], &capture[2]); // swap receiver for better formatting
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

fn search_email_log(server: String, ip_address: String) {    
    let shell_cmd = format!("cat /var/log/maillog | egrep \".+imap-login.+auth failed.+{}\"", ip_address);

    let output_string = ssh_command(&server, &shell_cmd);
    
    let num_strings = output_string.lines().count();

    // println!("{}", num_strings);

    let result;
    
    if num_strings > 0 {
        let re = Regex::new(r"^\w{3}\s+\d{1,2}\s\d{2}:\d{2}:\d{2}").unwrap();

        let matched = re.find(&output_string).unwrap();

        result = format!("{} IMAP failures from {}, the earliest being {}", num_strings, ip_address, matched.as_str());
    } else {
        result = format!("No IMAP failed logins from {}.", ip_address);
    }

    println!("{}", result);

    store(format!("Check IMAP failures on {} for {}", server, ip_address), result);
}

// fn ssh_command(server: String, email: &String) -> String {
fn ssh_command(server: &String, command: &String) -> String {
    // Set up the global SSH command

    let ssh_command = format!("ssh root@{} -p22222 '{}'", server, command);
    // let ssh_command = format!("ssh root@{} -p22222 'egrep -i \"Pass.+mailfrom.+envelope-from.+{}|orig_to=<{}>\" /var/log/maillog'", server, email, email);

    // If debug
    // println!("Global egrep regex to get all events:\n{}", ssh_command);    
    dbg!(&ssh_command);

    // Run the SSH command and capture the output
    let output = Command::new("bash")
                         .arg("-c")
                         .arg(&ssh_command)
                         .output()
                         .expect("Failed to execute SSH command");
    
    return String::from_utf8(output.stdout).unwrap().to_lowercase();
}

fn store(command: String, output: String) {
    let connection = Connection::open("history.db").unwrap();
    
    let query = format!("
        CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY, 
            command TEXT NOT NULL,
            output TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        INSERT INTO history ('command','output') VALUES ('{:?}','{:?}');
        ", command, output);
                
    connection.execute(query).unwrap();    
}



