/*

Field	Expected Value
Name	\x0ccodecrafters\x02io followed by a null byte (that's codecrafters.io encoded as a label sequence)
Type	1 encoded as a 2-byte big-endian int (corresponding to the "A" record type)
Class	1 encoded as a 2-byte big-endian int (corresponding to the "IN" record class)
TTL	Any value, encoded as a 4-byte big-endian int. For example: 60.
Length	4, encoded as a 2-byte big-endian int (corresponds to the length of the RDATA field)
Data	Any IP address, encoded as a 4-byte big-endian int. For example: \x08\x08\x08\x08 (that's 8.8.8.8 encoded as a 4-byte integer) */
use anyhow::{Result, Ok};

pub struct Answer {
    name: Vec<u8>,
    qtype: u16,
    qclass: u16,
    ttl: u32,
    length: u16,
    data: Vec<u8>,
}

impl Answer {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.name);
        bytes.push(0);
        bytes.push(((self.qtype >> 8) & 0xFF) as u8);
        bytes.push((self.qtype & 0xFF) as u8);
        bytes.push(((self.qclass >> 8) & 0xFF) as u8);
        bytes.push((self.qclass & 0xFF) as u8);
        bytes.push(((self.ttl >> 24) & 0xFF) as u8);
        bytes.push(((self.ttl >> 16) & 0xFF) as u8);
        bytes.push(((self.ttl >> 8) & 0xFF) as u8);
        bytes.push((self.ttl & 0xFF) as u8);
        bytes.push(((self.length >> 8) & 0xFF) as u8);
        bytes.push((self.length & 0xFF) as u8);
        bytes.extend(self.data);
        bytes
    }

    pub fn builder() -> AnswerBuilder {
        AnswerBuilder::new()
    }
}

pub struct AnswerBuilder {
    name: Option<Vec<u8>>,
    qtype: Option<u16>,
    qclass: Option<u16>,
    ttl: Option<u32>,
    length: Option<u16>,
    data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl AnswerBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            qtype: None,
            qclass: None,
            ttl: None,
            length: None,
            data: None,
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

    pub fn ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn length(mut self, length: u16) -> Self {
        self.length = Some(length);
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(mut self) -> Answer {
        if self.name.is_none() {
            self = self.name("codecrafters.io".to_owned()).unwrap();
        }
        Answer {
            name: self.name.unwrap(),
            qtype: self.qtype.unwrap_or(1),
            qclass: self.qclass.unwrap_or(1),
            ttl: self.ttl.unwrap_or(60),
            length: self.length.unwrap_or(4),
            data: self.data.unwrap_or(vec![8, 8, 8, 8]),
        }
    }
}