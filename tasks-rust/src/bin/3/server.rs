use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
    thread,
    time::Duration,
};

use crate::{ADDRESS, READ_TIMEOUT_S};

const NUMBER_OF_THREADS: u32 = 4;

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
        let mut stream = stream.unwrap();
        println!("Connected to client");

        stream
            .set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
            .expect("Could not set read timeout");

        // fills buffer with bytes of 32 which is spaces which is trimmed
        let mut buf = [32; 999];
        let client_message = stream.read(&mut buf);
        if client_message.is_err() {
            println!(
                "Could not read from client, {}",
                client_message.unwrap_err()
            );
            continue;
        }
        let string_buf = String::from_utf8(buf.to_vec()).expect("Could not parse from byte array");
        let calculation: Vec<&str> = string_buf.trim().split(" ").collect();
        let first_num_result = calculation[0].parse::<i32>();
        let second_num_result = calculation[2].parse::<i32>();
        if calculation.len() != 3
            || first_num_result.is_err()
            || second_num_result.is_err()
            || (calculation[1] != "-" && calculation[1] != "+")
        {
            println!("Invalid calculation");
            continue;
        }
        let result: i32;
        let first_num = first_num_result.unwrap();
        let second_num = second_num_result.unwrap();
        if calculation[1] == "+" {
            result = first_num + second_num;
        } else if calculation[1] == "-" {
            result = first_num - second_num;
        } else {
            println!("Invalid operator");
            continue;
        }
        println!(
            "Sending result of {} {} {} = {} to client",
            first_num, calculation[1], second_num, result
        );
        stream.write(&result.to_be_bytes()).unwrap();
        stream.flush().unwrap();

        stream
            .shutdown(Shutdown::Both)
            .expect("Could not shutdown stream")
    }
}
