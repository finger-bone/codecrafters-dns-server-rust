use crate::{message::{Header, Question}, Answer};

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
        let mut offset = 0 as usize;
        let header = Header::decode(&bytes[offset..]).unwrap();
        offset = 12;
        let mut questions = vec![];
        for _ in 0..header.qdcount {
            let question = Question::decode(&bytes[offset..]);
            offset += question.len();
            questions.push(question);
        }

        Message {
            header: header,
            questions: questions,
            answers: vec![]
        }
    }
}