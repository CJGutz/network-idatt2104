use std::net::UdpSocket;

use crate::{CLIENT_ADDRESS, SERVER_ADDRESS};

pub fn create_server() {
    println!("Server listening on {}", SERVER_ADDRESS);

    let socket = UdpSocket::bind(SERVER_ADDRESS).expect("Could not bind to address");

    loop {
        let error = calculate_from_stream(&socket);
        println!(
            "Errors: {}",
            if error.len() == 0 {
                "No errors"
            } else {
                &error
            }
        );
    }
}

fn calculate_from_stream(stream: &UdpSocket) -> String {
    // fills buffer with bytes of 32 which is spaces which is trimmed
    let mut buf = [32; 999];
    let client_message = stream.recv(&mut buf);
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
    stream
        .send_to(result.to_string().as_bytes(), CLIENT_ADDRESS)
        .unwrap();

    return "".to_string();
}
