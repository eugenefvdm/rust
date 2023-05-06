use dotenv::dotenv;
use math::convert_bytes;
use serde::{Deserialize};
use std::env;
use std::error::Error; // Used by the Async calls
use tokio::runtime::Runtime; // Used by the Async calls

enum Cmd {            
    Network(String, String, String), // e.g. network server add CP01.vander.host
}

fn main() {
    // Get the command and argument from the command line arguments
    let args: Vec<String> = env::args().collect();
    let command_str = &args[1];
    // Parse the command and argument using a match statement
    let command = match command_str.as_str() {                
        "network" => Cmd::Network(args[2].clone(), args[3].clone(), args[4].clone()),

        _ => {
            // If the command is not recognized, output an error message
            println!("Unrecognized command: {}", command_str);
                     
            return;
        }        
    };

    // Execute the appropriate command based on the parsed command
    match command {                
        Cmd::Network(command, verb, asset  ) => network(command, verb, asset),        
    }
    
}

fn network(command : String, verb :String, server: String) {
    match command.as_str() {
        "server" => {
            match verb.as_str() {
                "add" => {
                    let _result = add_server(server);
                },
                _ => println!("Unrecognized verb: {}", verb),
            }
        },
        _ => println!("Unrecognized command: {}", command),
    }
}

fn add_server(server : String) {
    store(server);
}

fn store(
    name: String,
    host: String,
    ip_address: String,
    port: String,
    username: String,
    password: String
) {
let connection = Connection::open("networks.db").unwrap();

let query = format!("
    CREATE TABLE IF NOT EXISTS history (
        id INTEGER PRIMARY KEY, 
        name TEXT NOT NULL,
        host TEXT,
        ip_address TEXT,
        port TEXT,
        username TEXT,
        password TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    INSERT INTO servers ('name','host', 'ip_address', 'port', 'username',) VALUES ('{:?}','{:?}','{:?}','{:?}','{:?});
    ", command, output);
            
connection.execute(query).unwrap();    
}



