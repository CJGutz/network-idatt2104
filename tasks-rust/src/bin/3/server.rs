use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    time::Duration,
};

use crate::{ADDRESS, READ_TIMEOUT_S};
use tasks_rust::workers::Workers;

const NUMBER_OF_THREADS: u32 = 4;

pub fn create_server() {
    let listener = TcpListener::bind(ADDRESS).expect("Could not bind to address");
    println!("Server listening on {}", ADDRESS);

    let mut workers = Workers::new(NUMBER_OF_THREADS);
    workers.start();

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
        workers.post(move || {
            let error = calculate_from_stream(&stream);
            if error.is_empty() {
                println!("Client disconnected successfully");
            } else {
                println!("Client disconnected with error: {}", error);
            }
            stream.write(error.as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
        });
    }

    workers.join();
}

fn calculate_from_stream(mut stream: &TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
        .expect("Could not set read timeout");

    // fills buffer with bytes of 32 which is spaces which is trimmed
    let mut buf = [32; 999];
    let client_message = stream.read(&mut buf);
    if client_message.is_err() {
        return format!(
            "Could not read from client, {}",
            client_message.unwrap_err()
        );
    }
    let string_buf = String::from_utf8(buf.to_vec()).expect("Could not parse from byte array");
    let calculation: Vec<&str> = string_buf.trim().split(" ").collect();
    let first_num_result = calculation[0].parse::<i32>();
    let second_num_result = calculation[2].parse::<i32>();
    if first_num_result.is_err() || second_num_result.is_err() {
        return "Invalid numbers".to_string();
    } else if calculation.len() != 3 {
        return "Invalid calculation length. Has to be 2 numbers with 1 operator".to_string();
    }
    let result: i32;
    let first_num = first_num_result.unwrap();
    let second_num = second_num_result.unwrap();
    if calculation[1] == "+" {
        result = first_num + second_num;
    } else if calculation[1] == "-" {
        result = first_num - second_num;
    } else {
        return "Invalid operator. Valid are + and -".to_string();
    }
    println!(
        "Sending result of {} {} {} = {} to client",
        first_num, calculation[1], second_num, result
    );
    stream.write(result.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();

    return "".to_string();
}
