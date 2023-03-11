use std::{
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, Shutdown},
    sync::{Arc, Mutex},
};

use websocket_server::{send_message, websocket_connect};

use crate::websocket_server::decode_message;

mod websocket_server;

fn main() {
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let buf = &mut [32; 1000];
                stream.read(buf).expect("Could not read from stream");
                let request_string = String::from_utf8_lossy(&buf[..]);
                let request_string = request_string.trim();
                if request_string.contains("Sec-WebSocket-Key")
                    && request_string.contains("Upgrade: websocket")
                    && request_string.contains("Connection: Upgrade")
                {
                    websocket_connect(&request_string, &stream);
                    let clients_clone = clients.clone();
                    std::thread::spawn(move || {
                        send_recieved_message_to_connections(&stream, clients_clone);
                    });
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn send_recieved_message_to_connections(
    mut stream: &TcpStream,
    clients: Arc<Mutex<Vec<TcpStream>>>,
) {
    let locked_clients = clients.lock();
    let stream_clone = stream.try_clone();
    if locked_clients.is_err() || stream_clone.is_err() {
        println!("Could not lock clients or clone stream");
        return;
    }
    locked_clients.unwrap().push(stream_clone.unwrap());

    println!("Added client to list of clients");
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        stream
            .read(&mut buf)
            .expect("Could not read from stream in loop");

        let message = decode_message(buf);
        for client in clients.lock().unwrap().iter() {
            let result = send_message(&message, client);
            if result.is_err() {
                println!("Could not send message to client. Disconected");
                stream.shutdown(Shutdown::Both).expect("Could not shutdown stream");
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
