use crate::server::ADDRESS;
use std::{
    io::stdin,
    net::{Shutdown, TcpStream},
};

use crate::defer::defer;

pub fn create_client() {
    let stream = TcpStream::connect(ADDRESS).expect("Could not connect to server");
    defer(|| {
        stream
            .shutdown(Shutdown::Both)
            .expect("Could not shutdown stream");
    });
    println!("Client started");

    println!("First number: ");
    let buffer = &mut String::new();
    stdin().read_line(buffer).expect("Recieved no input");
}
