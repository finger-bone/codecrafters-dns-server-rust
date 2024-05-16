use crate::{message::{Header, Question}, Answer};

pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answer: Answer,
}

impl Message {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.encode());
        for question in self.questions {
            bytes.extend(question.encode());
        }
        bytes.extend(self.answer.encode());
        bytes
    }
}