mod message;

use std::net::UdpSocket;

use crate::message::*;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let request = Message::decode(&buf);

                let response = Message {
                    header: Header::builder()
                        .qdcount(1)
                        .unwrap()
                        .ancount(1)
                        .unwrap()
                        .id(request.header.id)
                        .unwrap()
                        .opcode(request.header.opcode)
                        .unwrap()
                        .rd(request.header.rd)
                        .unwrap()
                        .rcode(if request.header.opcode == 0 { 0 } else { 4 })
                        .unwrap()
                        .build(),
                    questions: vec![
                        Question::builder().build()
                    ],
                    answers: vec![
                        Answer::builder().build()
                    ],
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
