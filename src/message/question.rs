use anyhow::{Result, Ok};

pub struct Question {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
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

    pub fn decode(bytes: &[u8]) -> Question {
        let mut name = vec![];
        let mut i = 0;
        loop {
            let len = bytes[i] as usize;
            if len == 0 {
                break;
            }
            if i > 0 {
                name.push(b'.');
            }
            name.extend_from_slice(&bytes[i + 1..i + 1 + len]);
            i += len + 1;
        }
        Question {
            name,
            qtype: ((bytes[i + 1] as u16) << 8) | bytes[i + 2] as u16,
            qclass: ((bytes[i + 3] as u16) << 8) | bytes[i + 4] as u16,
        }
    }

    pub fn len(&self) -> usize {
        self.name.len() + 6
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

    pub fn name_bytes(mut self, name: Vec<u8>) -> Result<Self> {
        self.name = Some(name);
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