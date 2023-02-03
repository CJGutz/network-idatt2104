use std::{
    io::Error,
    io::{stdin, Read, Write},
    net::{Shutdown, TcpStream},
    thread,
    time::Duration,
};

use crate::{ADDRESS, READ_TIMEOUT_S};

pub fn create_client() {
    let mut stream = TcpStream::connect(ADDRESS).expect("Could not connect to server");

    println!("Client started");

    stream
        .set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_S)))
        .expect("Could not set read timout");

    let n = &mut String::new();
    stdin().read_line(n).expect("msg");
    send_to_server(&mut stream);

    stream
        .shutdown(Shutdown::Both)
        .expect("Could not shutdown tcp stream");
}

fn send_to_server(stream: &mut TcpStream) {
    let write_buffer = get_user_calculation();
    stream
        .write(write_buffer.as_bytes())
        .expect("Could not write to stream");
}

fn get_from_server(stream: &mut TcpStream) {
    let read_buffer = &mut String::new();
    stream
        .read_to_string(read_buffer)
        .expect("Could not read from stream");
    println!("Message: {}", read_buffer);
}

fn get_user_calculation() -> String {
    let mut calculation = String::new();
    while calculation.is_empty() {
        print!("First number: ");
        let first_number = get_valid_input(None).expect("Recieved no input for first number");

        print!("Operator: [+ -] ");
        let valid = vec!["+", "-"];
        let mut operator = get_valid_input(Some(&valid));
        while operator.is_err() {
            print!("Invalid operator, please try again: ");
            operator = get_valid_input(Some(&valid));
        }
        let operator = operator.unwrap();

        print!("Second number: ");
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
                return Ok(input);
            }
        }
        Err(e) => println!("Could not read from stream, {}", e),
    };
    Err(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid input",
    ))
}
