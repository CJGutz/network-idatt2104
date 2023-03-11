/*

Request
    Sec-Websocket-Key: base 64 (Random bytestring from client)
    Sec-Websocket-Version: 13

Response
    Status code: 101 Switching Protocols
    Upgrade: websocket
    Connection: Upgrade
    Sec-Websocket-Accept: base 64
        Base64Encode(SHA1(key+'258EAFA5-E914-47DA-95CA-C5AB0DC85B11'))

Messages
    1. byte:
        0x81: Text
        0x82: Binary
        0x88: Close
        0x89: Ping
        0x8A: Pong
    2. byte:
        If small message (< 127)
            1. bit: If masked or not
            2-8: Payload length
    If client:
        3. -> 6. byte:
            bytes used for masking
            Server shall not mask response
            Uses XOR to mask
    Last bytes:
        Payload data

*/

use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1::{Digest, Sha1};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    ops::Add,
};

pub struct WebSocket {
    pub address: SocketAddr,
    pub stream: Option<TcpStream>,
}

impl WebSocket {
    pub fn wait_for_connection(address: SocketAddr) -> Result<Self, String> {
        let listener = TcpListener::bind(address)
            .expect(format!("Could not connect to address {}", address).as_str());
        println!("Server listening on {}", address);
        let mut websocket = Self {
            address,
            stream: None,
        };
        for stream in listener.incoming() {
            let mut stream = stream.expect("Could not connect to client");
            let result = websocket.create_websocket_connection(&mut stream);
            if result.is_ok() {
                websocket.stream = Some(stream);
                break;
            } else {
                return Err(result.err().unwrap().to_string());
            }
        }
        return Ok(websocket);
    }

    fn create_websocket_connection(&self, stream: &mut TcpStream) -> Result<(), &str> {
        // Read client request
        let mut buf = [0; 1024];
        stream.read(&mut buf).expect("Could not read from stream");
        let request_string = String::from_utf8_lossy(&buf[..]);
        let request_lines: Vec<&str> = request_string.trim().split("\r\n").collect();

        // Extract Sec-WebSocket-Key header
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
        let sec_accept = if !sec_key.is_empty() {
            let mut hasher = Sha1::new();
            hasher.update(sec_key.as_bytes());
            let digest = hasher.finalize();
            let encoded = STANDARD.encode(digest);
            encoded
        } else {
            "".to_string()
        };
        let response = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", sec_accept);
        stream.write(response.as_bytes()).unwrap();

        Ok(())
    }

    pub fn send_message(&self, message: &str) {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(0b10000001);
        let size = message.len();
        buffer.push((size | 0b00000000) as u8);
        for byte in message.as_bytes() {
            buffer.push(*byte);
        }
        self.stream
            .as_ref()
            .unwrap()
            .write_all(&buffer)
            .expect("Could not write to stream");
    }

    pub fn on_message(&self, callback: fn(&TcpStream)) {
        println!("Connected to client");
        callback(&self.stream.as_ref().unwrap());
    }
}
