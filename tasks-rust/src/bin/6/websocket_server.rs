use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1::{Digest, Sha1};
use std::{io::Write, net::TcpStream};

pub fn websocket_connect(opening_request: &str, mut stream: &TcpStream) {
    let sec_accept_encoded = get_sec_accept_encoded(opening_request);

    let response = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", sec_accept_encoded);
    stream.write(response.as_bytes()).unwrap();
}

fn get_sec_accept_encoded(request: &str) -> String {
    let request_lines: Vec<&str> = request.trim().split("\r\n").collect();
    let key_line = request_lines
        .iter()
        .find(|line| line.contains("Sec-WebSocket-Key"));
    let sec_key = match key_line {
        Some(line) => {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() > 1 {
                format!("{}{}", parts[1], "258EAFA5-E914-47DA-95CA-C5AB0DC85B11")
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    };

    // Compute Sec-WebSocket-Accept header value
    return if !sec_key.is_empty() {
        let mut hasher = Sha1::new();
        hasher.update(sec_key.as_bytes());
        let digest = hasher.finalize();
        let encoded = STANDARD.encode(digest);
        encoded
    } else {
        "".to_string()
    };
}

pub fn send_message(message: &str, mut stream: &TcpStream) -> Result<(), std::io::Error> {
    let mut buf: Vec<u8> = Vec::new();
    buf.push(0b10000001);
    buf.push((message.len() & 0b01111111) as u8);
    for byte in message.as_bytes() {
        buf.push(*byte);
    }
    stream.write_all(&buf)
}

pub fn decode_message(bytes: [u8; 1024]) -> String {
    let length = bytes[1] as usize & 0b01111111;
    let mask_start = 2;
    let data_start = mask_start + 4;

    let mut message = String::new();
    for i in data_start..data_start + length {
        if bytes[i] > 1 {
            let byte = bytes[i] ^ bytes[mask_start + (i - data_start) % 4];
            message.push(byte as char);
        }
    }

    message
}
