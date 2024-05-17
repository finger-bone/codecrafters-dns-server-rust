use std::net::UdpSocket;

use crate::message::*;

pub fn build_response(request: Message) -> Message {
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
            .data(vec![8, 8, 8, 8])
            .unwrap()
            .build();
        questions.push(response_question);
        answers.push(response_answer);
    }

    Message {
        header: Header::builder()
            .qdcount(questions.len() as u16)
            .unwrap()
            .ancount(answers.len() as u16)
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
        questions: questions,
        answers: answers,
    }
}

pub fn build_response_forward(request: Message, resolver: String, socket: &UdpSocket) -> Message {
    let mut answers = vec![];
    let mut questions = vec![];

    for question in request.questions {
        
        let relay: Message = Message {
            header: request.header.clone(),
            questions: vec![question.clone()],
            answers: vec![],
        };

        println!("Relaying request to resolver: {}", resolver);
        let relay_bytes = relay.encode();

        socket
            .send_to(&relay_bytes, &resolver)
            .expect("Failed to send request to resolver");

        let mut relayed_buffer: [u8; 512] = [0; 512];
        socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("Failed to set read timeout");

        let size= socket
            .recv(&mut relayed_buffer)
            .expect("Failed to receive response from resolver");

        let response = Message::decode(&relayed_buffer[..size]);

        println!("Got response from resolver.");

        questions.extend(vec![question.clone()]);
        answers.extend(if response.answers.len() > 0 {
            response.answers
        } else {
            vec![Answer::builder()
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
                .data(vec![8, 8, 8, 8])
                .unwrap()
                .build()]
        });
    }

    Message {
        header: Header::builder()
            .qdcount(questions.len() as u16)
            .unwrap()
            .ancount(answers.len() as u16)
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
        questions: questions,
        answers: answers,
    }
}