use std::{
    fs,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    ops::Add,
    process::Command,
};
use urlencoding::decode;

use tasks_rust::workers::Workers;

const ADDRESS: &str = "127.0.0.1:8080";
const NUMBER_OF_THREADS: u32 = 4;

fn main() {
    println!("Server started on server {}", ADDRESS);
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
    let buf = &mut [0; 200];
    match stream.read(buf) {
        Ok(_) => (),
        Err(_) => println!("Connection lost"),
    };
    let request_string = buf.iter().map(|&x| x as char).collect::<String>();

    let code = parse_url(&request_string);

    let result = match code {
        Ok(code) => run_code(code.as_str()),
        Err(e) => String::from(format!("There was an error parsing your request {:?}", e)),
    };

    let index_file = fs::read_to_string("./src/bin/5/index.html")
        .expect("Could not read html file")
        .add(format!("<pre>{}</pre></body></html>", result).as_str());
    let response = format!(
        "HTTP/1.1 200 OK\nContent-Type: text/html; charset=utf-8\n\n{}",
        index_file
    );

    stream
        .write(response.as_bytes())
        .expect("Could not write to stream.");

    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap_or_default();
}

fn parse_url(url: &str) -> Result<String, String> {
    let start = match url.find("code=") {
        Some(size) => size,
        None => return Err("Request did not contain the code attribute".to_string()),
    } + 5;
    let end = url
        .find(" HTTP/")
        .unwrap_or(url.find("\n").unwrap_or(url.len()));
    let code = url[start..end].to_string();
    let decoded = decode(code.as_str())
        .expect("Code could not be decoded")
        .into_owned()
        .replace("+", " ");
    Ok(decoded)
}

fn run_code(code: &str) -> String {
    let command = format!("printf {:?} > main.rs && rustc main.rs && ./main", code);
    let result = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("rust:alpine3.17")
        .arg("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Could not run docker");

    let error = result.stderr.iter().map(|&x| x as char).collect::<String>();
    let output = result.stdout.iter().map(|&x| x as char).collect::<String>();
    if error.len() > 0 {
        return error;
    }
    output
}
