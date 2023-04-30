use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use tokio::runtime::Runtime;

fn main() {    
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        match get_products().await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        };
    });
}

async fn get_products() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let url = env::var("WHMCS_URL").expect("WHMCS_URL not set");
    let username = env::var("WHMCS_IDENTIFIER").expect("WHMCS_IDENTIFIER not set");
    let password = env::var("WHMCS_SECRET").expect("WHMCS_SECRET not set");
    
    let client = reqwest::Client::new();
    let url = url + "includes/api.php";
    
    let mut params = HashMap::new();
    params.insert("action", "GetProducts");
    params.insert("username", &username);
    params.insert("password", &password);
    params.insert("pid", "1");
    params.insert("responsetype", "json");

    let response = client.post(url)
        .form(&params)
        .send()
        .await?;
        
    let content = response.text().await?;
    println!("{}", content); // or process the content as required
    
    Ok(())
}
