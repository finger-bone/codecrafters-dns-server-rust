use crate::{message::{Header, Question}, Answer};

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.encode());
        for question in self.questions {
            bytes.extend(question.encode());
        }
        for answer in self.answers {
            bytes.extend(answer.encode());
        }
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Self {
        let header = Header::decode(bytes).unwrap();
        let header_offset = 12;
        let questions = Question::decode(
            &bytes[header_offset..], 
            header.qdcount as usize,
            header_offset,
        );

        Message {
            header: header,
            questions: questions,
            answers: vec![]
        }
    }
}