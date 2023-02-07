use std::{io::stdin, io::Error, net::UdpSocket, time::Duration};

use crate::{CLIENT_ADDRESS, READ_TIMEOUT_S, SERVER_ADDRESS};

pub fn create_client() {
    println!("Client started");

    let mut should_continue = true;
    while should_continue {
        println!("Enter calculation: ");
        let calculation = get_user_calculation();
        send_to_server(calculation);

        println!("Do you want to continue? [y/N]");
        should_continue = get_valid_input(Some(&vec!["y", "Yes"])).is_ok();
    }
}

fn send_to_server(message: String) {
    println!("Client listening on {}", SERVER_ADDRESS);
    let mut stream = UdpSocket::bind(CLIENT_ADDRESS).expect("Could not connect to server");
    stream
        .set_write_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
        .expect("Could not set read timout");
    stream
        .send_to(message.as_bytes(), SERVER_ADDRESS)
        .expect("Could not write to stream");
    let response = get_from_server(&mut stream);
    println!("Response: {}", response);
}

fn get_from_server(stream: &mut UdpSocket) -> String {
    let mut read_buffer = [32; 999];
    stream
        .recv(&mut read_buffer)
        .expect("Could not read from stream");
    return read_buffer
        .iter()
        .map(|&x| x as char)
        .collect::<String>()
        .trim()
        .to_string();
}

fn get_user_calculation() -> String {
    let mut calculation = String::new();
    while calculation.is_empty() {
        println!("First number: ");
        let first_number = match get_valid_input(None) {
            Ok(num) => num,
            Err(err) => {
                println!("Recieved no input for first number, {}", err);
                continue;
            }
        };

        println!("Operator: [+ -] ");
        let valid = vec!["+", "-"];
        let mut operator = get_valid_input(Some(&valid));
        while operator.is_err() {
            println!("Invalid operator, please try again: ");
            operator = get_valid_input(Some(&valid));
        }
        let operator = operator.unwrap();

        println!("Second number: ");
        let second_number = get_valid_input(None).expect("Recieved no input for second number");

        if first_number.parse::<i32>().is_ok() && second_number.parse::<i32>().is_ok() {
            calculation = format!("{} {} {}", first_number, operator, second_number);
        } else {
            println!("Invalid input, please try again");
        }
    }

    return calculation;
}

fn get_valid_input(valid: Option<&Vec<&str>>) -> Result<String, Error> {
    let buf = &mut String::new();
    match stdin().read_line(buf) {
        Ok(_) => {
            let input = buf.trim().to_lowercase();
            if valid.is_some() {
                if valid.unwrap().contains(&input.as_str()) {
                    return Ok(input);
                }
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Input is not in the valid list",
                ));
            }
            return Ok(input);
        }
        Err(e) => println!("Could not read from standard input, {}", e),
    };
    Err(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid input",
    ))
}
