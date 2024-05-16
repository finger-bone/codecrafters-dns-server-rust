/*

Field	Expected value
Name	\x0ccodecrafters\x02io followed by a null byte (that's codecrafters.io encoded as a label sequence)
Type	1 encoded as a 2-byte big-endian int (corresponding to the "A" record type)
Class	1 encoded as a 2-byte big-endian int (corresponding to the "IN" record class)
Make sure to update the QDCOUNT field in the header section accordingly, and remember to set the id to 1234. */

use anyhow::{Result, Ok};

pub struct Question {
    name: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

impl Question {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.name);
        bytes.push(0);
        bytes.push(((self.qtype >> 8) & 0xFF) as u8);
        bytes.push((self.qtype & 0xFF) as u8);
        bytes.push(((self.qclass >> 8) & 0xFF) as u8);
        bytes.push((self.qclass & 0xFF) as u8);
        bytes
    }

    pub fn builder() -> QuestionBuilder {
        QuestionBuilder::new()
    }
}

pub struct QuestionBuilder {
    name: Option<Vec<u8>>,
    qtype: Option<u16>,
    qclass: Option<u16>,
}

#[allow(dead_code)]
impl QuestionBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            qtype: None,
            qclass: None,
        }
    }

    pub fn name(mut self, name: String) -> Result<Self> {
        let mut name_bytes = vec![];
        for label in name.split('.') {
            name_bytes.push(label.len() as u8);
            name_bytes.extend(label.as_bytes());
        }
        self.name = Some(name_bytes);
        Ok(self)
    }

    pub fn qtype(mut self, qtype: u16) -> Self {
        self.qtype = Some(qtype);
        self
    }

    pub fn qclass(mut self, qclass: u16) -> Self {
        self.qclass = Some(qclass);
        self
    }

    pub fn build(mut self) -> Question {
        if self.name.is_none() {
            self = self.name("codecrafters.io".to_owned()).unwrap();
        }
        Question {
            name: self.name.unwrap(),
            qtype: self.qtype.unwrap_or(1),
            qclass: self.qclass.unwrap_or(1),
        }
    }
}