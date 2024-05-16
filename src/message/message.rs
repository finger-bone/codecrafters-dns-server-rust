use crate::{message::Header, question::Question};

pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
}

impl Message {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.encode());
        for question in self.questions {
            bytes.extend(question.encode());
        }
        bytes
    }
}