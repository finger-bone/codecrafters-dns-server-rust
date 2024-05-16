mod message;

use std::net::UdpSocket;

use crate::{header::Header, message::{message::Message, *}, question::Question};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = Message {
                    header: Header::builder().qdcount(1).unwrap().build(),
                    questions: vec![
                        Question::builder().build()
                    ]
                };
                udp_socket
                    .send_to(&response.encode(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
