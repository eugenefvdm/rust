use dotenv::dotenv;
use math::convert_bytes;
use serde::{Deserialize};
use std::env;
use std::error::Error; // Used by the Async calls
use tokio::runtime::Runtime; // Used by the Async calls

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    values: UserValues,
}

#[derive(Debug, Deserialize)]
struct UserValues {
    home_byte_quota_used: Vec<String>,    
}

#[derive(Debug, Deserialize)]
struct ListUsersResponse {
    data: Vec<User>,
    status: String,    
}

enum Cmd {    
    Demo(String),
    Mailboxes(String, String),    
}

fn main() {
    // Get the command and argument from the command line arguments
    let args: Vec<String> = env::args().collect();
    let command_str = &args[1];

    // Parse the command and argument using a match statement
    let command = match command_str.as_str() {
        "demo" => Cmd::Demo(args[2].clone()),
        "mailboxes" => Cmd::Mailboxes(args[2].clone(), args[3].clone()),
        _ => {
            // If the command is not recognized, output an error message
            println!("Unrecognized command: {}", command_str);
            println!("List of commands:");
            println!(" demo list-users");
            println!(" mailboxes <domain> <server>");            
            return;
        }
    };

    // Execute the appropriate command based on the parsed command
    match command {        
        Cmd::Mailboxes(domain, server) => mailboxes(domain, server),
        Cmd::Demo(command) => demo(command),
    }
    
}

fn mailboxes(domain: String, server: String) {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        match virtualmin_api(domain, server).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        };
    });            
}

async fn virtualmin_api(domain: String, server: String) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::new();

    let url = 
        format!("https://{}:10000/virtual-server/remote.cgi?program=list-users&domain={}&multiline&json=1", server, domain);
    
    let res = client
        .get(url)
        .basic_auth(
            env::var("VIRTUALMIN_USERNAME").expect("VIRTUALMIN_USERNAME not set"),
            Some(env::var("VIRTUALMIN_PASSWORD").expect("VIRTUALMIN_PASSWORD not set"))
        )
        .send()
        .await?;

    let body = res.text().await?;
        
    // println!("{}", body);

    deserialize_list_users(body)?;
    
    Ok(())
}

fn demo(command: String) {
    match command.as_str() {
        "list-users" => list_users_demo().unwrap(),
        _ => println!("Unrecognized demo command: {}", command),
    }
}

fn list_users_demo() -> serde_json::Result<()> {
    let json_str = r#"{
        "data": [
            {
                "name": "user1@example.com",
                "values": {
                    "encrypted_password": [
                        "$6$30645809$Xq4tE5smGBUyDt7e1hzxJ5ZAt6H/Z2mEVjnzkaFNt5ZLhONoN4XxbEfVBR.HWznKkv8hT6p.W4Nr0EdhF4lC4/"
                    ],
                    "home_byte_quota_used": [
                        "43446272"
                    ]
                }
            },
            {
                "name": "user2@example.com",
                "values": {
                    "encrypted_password": [
                        "$6$76023015$fZ9Kilin49jX7645IzhK4Cf5uFDdpfK6RcXx9bfcc4dLpVspL1ik0UrtviY5Jdhzl1Uxeu3l2N.AsPQ9Si5Ww0"
                    ],
                    "home_byte_quota_used": [
                        "1458176"
                    ]
                }
            }
        ],
        "status": "success",
        "command": "list-users"
    }"#;

    deserialize_list_users(json_str.to_string())

}

fn deserialize_list_users(json_str: String ) -> serde_json::Result<()> {    
    let list_users_response: ListUsersResponse = serde_json::from_str(&json_str)?;

    for user in list_users_response.data {
        println!(
            "Name:{}, Used: {}",
            user.name,
            convert_bytes(user.values.home_byte_quota_used[0].parse::<u64>().unwrap()),
        );
    }
    println!("Status: {}", list_users_response.status);

    Ok(())
}