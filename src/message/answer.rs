use anyhow::{Result, Ok};

#[derive(Debug)]
pub struct Answer {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
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

    pub fn decode(bytes: &[u8], ancount: usize) -> Vec<Answer> {
        let mut cur = 0;
        let mut cnt = 0;

        let mut answers = vec![];

        while cnt < ancount {
            let mut name: Vec<u8> = vec![];
            while bytes[cur] != 0 {
                let length = bytes[cur] as usize;
                name.extend(
                    &bytes[cur..=cur+length]
                );
                cur += length + 1;
            }
            cur += 1;
            let atype = ((bytes[cur] as u16) << 8) | bytes[cur + 1] as u16;
            cur += 2;
            let aclass = ((bytes[cur] as u16) << 8) | bytes[cur + 1] as u16;
            cur += 2;
            let ttl = ((bytes[cur] as u32) << 24) | ((bytes[cur + 1] as u32) << 16) | ((bytes[cur + 2] as u32) << 8) | bytes[cur + 3] as u32;
            cur += 4;
            let length = ((bytes[cur] as u16) << 8) | bytes[cur + 1] as u16;
            cur += 2;
            let data = bytes[cur..cur + length as usize].to_vec();
            cur += length as usize + 1;
            answers.push(Answer {
                name: name,
                qtype: atype,
                qclass: aclass,
                ttl: ttl,
                length: length,
                data: data,
            });
            cnt += 1;
        }

        answers
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

    pub fn name_bytes(mut self, name: Vec<u8>) -> Result<Self> {
        self.name = Some(name);
        Ok(self)
    }

    pub fn qtype(mut self, qtype: u16) -> Result<Self> {
        self.qtype = Some(qtype);
        Ok(self)
    }

    pub fn qclass(mut self, qclass: u16) -> Result<Self> {
        self.qclass = Some(qclass);
        Ok(self)
    }

    pub fn ttl(mut self, ttl: u32) -> Result<Self> {
        self.ttl = Some(ttl);
        Ok(self)
    }

    pub fn length(mut self, length: u16) -> Result<Self> {
        self.length = Some(length);
        Ok(self)
    }

    pub fn data(mut self, data: Vec<u8>) -> Result<Self> {
        self.data = Some(data);
        Ok(self)
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