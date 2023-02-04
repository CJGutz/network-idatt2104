use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    ops::Add,
};

use crate::workers::Workers;

mod workers;

const ADDRESS: &str = "127.0.0.1:8080";
const NUMBER_OF_THREADS: u32 = 4;

fn main() {
    println!("Server started");
    let mut workers = Workers::new(NUMBER_OF_THREADS);
    workers.start();

    let listener = TcpListener::bind(ADDRESS).expect(
        format!(
            "Could not bind to {}. Maybe it is already allocated",
            ADDRESS
        )
        .as_str(),
    );

    for stream in listener.incoming() {
        if stream.is_err() {
            println!("Could not connect to client!");
        }
        let mut stream = stream.unwrap();

        workers.post(move || {
            handle_tcp_stream(&mut stream);
        });
    }

    workers.join();
}

fn handle_tcp_stream(stream: &mut TcpStream) {
    stream.set_read_timeout(None).unwrap();

    let buf = &mut [0; 200];
    match stream.read(buf) {
        Ok(_) => (),
        Err(_) => println!("Connection lost"),
    };
    let request_string = buf
        .iter()
        .map(|&x| x as char)
        .collect::<String>()
        .trim()
        .to_string();

    println!("Request:\n{}", request_string);

    let mut headers: Vec<&str> = request_string.split("\n").collect();
    headers.remove(0);

    let mut response = String::from("HTTP/1.1 200 OK\nContent-Type: text/html; charset=utf-8\n\n");
    response =
        response.add(
            format!(
            "<html><body><h1>Welcome to this site!</h1><p>Headers:</p><ul>{}</ul></body></html>",
            headers.iter().map(|header| format!("<li>{}</li>", header)).collect::<String>()
        )
            .as_str(),
        );

    // Send response
    stream
        .write(response.as_bytes())
        .expect("Could not write to stream.");

    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}
