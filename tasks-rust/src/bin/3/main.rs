use std::{
    io::Read,
    net::{Shutdown, TcpListener},
    thread,
    time::Duration,
};

const ADDRESS: &str = "127.0.0.1:8080";
const NUMBER_OF_THREADS: u32 = 4;

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("Could not bind to address");

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
        let mut stream = stream.unwrap();
        println!("Connected to client");
        let buf = &mut String::new();
        println!(
            "Message: {}",
            stream.read_to_string(buf).expect("No message recieved")
        );
        if stream.shutdown(Shutdown::Both).is_ok() {
            println!("Tcp stream shutdown");
        }
    }
}
