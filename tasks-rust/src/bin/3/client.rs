use std::{
    io::Error,
    io::{stdin, Read, Write},
    net::{Shutdown, TcpStream},
    time::Duration,
};

use crate::{ADDRESS, READ_TIMEOUT_S};

pub fn create_client() {
    println!("Client started");

    let mut should_continue = true;
    while should_continue {
        println!("Enter calculation: ");
        let calculation = get_user_calculation();
        send_to_server(calculation);

        println!("Do you want to continue? [Y/n]");
        should_continue = get_valid_input(Some(&vec!["y", "Y", "Yes"])).is_ok();
    }
}

fn send_to_server(message: String) {
    let mut stream = TcpStream::connect(ADDRESS).expect("Could not connect to server");
    stream
        .set_write_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
        .expect("Could not set read timout");
    stream
        .write_all(message.as_bytes())
        .expect("Could not write to stream");
    let response = get_from_server(&mut stream);
    println!("Response: {}", response);
    stream
        .shutdown(Shutdown::Both)
        .expect("Could not shutdown tcp stream");
}

fn get_from_server(stream: &mut TcpStream) -> String {
    let read_buffer = &mut String::new();
    stream
        .read_to_string(read_buffer)
        .expect("Could not read from stream");
    return read_buffer.to_string();
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

        if first_number.parse::<u32>().is_ok() && second_number.parse::<u32>().is_ok() {
            calculation = format!("{} {} {}", first_number, operator, second_number);
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
