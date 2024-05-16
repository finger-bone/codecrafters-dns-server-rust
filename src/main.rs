mod message;

use std::net::UdpSocket;

use crate::message::*;

fn build_response(request: Message) -> Message {
    let mut answers = vec![];
    let mut questions = vec![];

    for question in request.questions {

        let response_question = Question::builder()
            .name_bytes(question.name.clone())
            .unwrap()
            .build();
        let response_answer = Answer::builder()
            .name_bytes(question.name.clone())
            .unwrap()
            .qtype(question.qtype)
            .unwrap()
            .qclass(question.qclass)
            .unwrap()
            .ttl(60)
            .unwrap()
            .length(4)
            .unwrap()
            .data(vec![127, 0, 0, 1])
            .unwrap()
            .build();
        questions.push(response_question);
        answers.push(response_answer);
    }

    Message {
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
        questions,
        answers,
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let request = Message::decode(&buf);
                println!("Got Request {:#?}", request);

                let response = build_response(request);
                println!("Responding With: {:#?}", response);

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
