use client::create_client;
use server::create_server;
use std::io::stdin;

mod client;
mod server;

pub const READ_TIMEOUT_S: u64 = 2;
pub const SERVER_ADDRESS: &str = "[::]:8000";
pub const CLIENT_ADDRESS: &str = "[::]:8080";

fn main() {
    println!("Select service: [Server, Client]");
    let server_alias: Vec<&str> = vec!["server", "s"];
    let client_alias: Vec<&str> = vec!["client", "c"];

    let buffer = &mut String::new();
    stdin().read_line(buffer).expect("Recieved no input");
    let buffer = buffer.trim().to_lowercase();
    let buffer = &mut buffer.as_str();

    if server_alias.contains(buffer) {
        println!("Starting server");
        create_server();
    } else if client_alias.contains(buffer) {
        println!("Starting client");
        create_client();
    } else {
        println!("Invalid input");
    }
}
