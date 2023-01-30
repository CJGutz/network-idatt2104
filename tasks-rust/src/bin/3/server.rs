use std::{
    io::Read,
    net::{Shutdown, TcpListener},
    thread,
    time::Duration,
};

use crate::defer::defer;

pub const ADDRESS: &str = "127.0.0.1:8080";
const NUMBER_OF_THREADS: u32 = 4;
const READ_TIMEOUT_S: u64 = 30;

pub fn create_server() {
    let listener = TcpListener::bind(ADDRESS).expect("Could not bind to address");
    println!("Server listening on {}", ADDRESS);

    for _ in 0..NUMBER_OF_THREADS {
        let listener = listener.try_clone().expect("Could not clone listener");
        thread::spawn(move || {
            start_listener(listener);
        });
    }

    thread::sleep(Duration::from_secs(100));
}

fn start_listener(listener: TcpListener) {
    for stream in listener.incoming() {
        if stream.is_err() {
            println!(
                "Could not connect to client request, {}",
                stream.unwrap_err()
            );
            continue;
        }
        println!("Connected to client");
        let buf = &mut String::new();
        let mut stream = stream.unwrap();

        defer(|| {
            if stream.shutdown(Shutdown::Both).is_ok() {
                println!("Tcp stream shutdown");
            } else {
                println!("Could not shutdown tcp stream");
            }
        });

        stream
            .set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
            .expect("Could not set read timeout");
        match stream.read_to_string(buf) {
            Ok(_) => println!("Message: {}", buf),
            Err(e) => match e.kind() {
                std::io::ErrorKind::TimedOut => println!("Read timed out"),
                _ => println!("Could not read from stream, {}", e),
            },
        }
    }
}
