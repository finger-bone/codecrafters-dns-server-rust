pub mod response;
pub mod message;

use std::net::UdpSocket;
use std::env;
use crate::{message::*, response::{build_response, build_response_forward}};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    let args: Vec<String> = env::args().collect();

    loop {
        println!("Waiting for data...");
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let request = Message::decode(&buf);

                let response = if args.len() == 1 {
                    println!("Directly building response.");
                    build_response(request)
                } else {
                    println!("Forwarding request to resolver.");
                    build_response_forward(request, args[2].clone(), &udp_socket)
                };

                println!("Response built, sending to {}", source);

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
