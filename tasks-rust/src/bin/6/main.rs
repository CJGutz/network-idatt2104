use std::{
    io::{Read},
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use websocket_server::WebSocket;

mod websocket_server;

fn main() {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);
    let websocket = WebSocket::wait_for_connection(address).unwrap();
    let mut stream = websocket.stream.as_ref().unwrap();
    println!("Connected to {}", websocket.address);
    println!("Sending message to client");
    websocket.send_message("Hello from server");
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}
