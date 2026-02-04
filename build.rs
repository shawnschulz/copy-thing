// build.rs
use std::fs;
use std::env;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    let ip_address = env::var("IP_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    
    let template = fs::read_to_string("index.html.template")
        .expect("Failed to read index.html.template");
    
    let output = template.replace("{ip_address}", &ip_address);
    
    fs::write("index.html", output)
        .expect("Failed to write index.html");
    
    println!("cargo:rerun-if-changed=index.html.template");
    println!("cargo:rerun-if-env-changed=IP_ADDRESS");
}
