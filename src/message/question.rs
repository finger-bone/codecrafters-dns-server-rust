use std::collections::HashMap;

use anyhow::{Result, Ok};

#[derive(Debug)]
#[derive(Clone)]
pub struct Question {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
}

#[allow(dead_code)]
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

    pub fn decode(bytes: &[u8], qncount: usize, header_offset: usize) -> (Vec<Question>, usize) {
        #[derive(Debug)]
        enum LabelOrPointer {
            Label(Vec<u8>),
            Pointer(usize),
            QType(Vec<u8>),
            QClass(Vec<u8>),
            Stop,
        }

        let mut tokens: Vec<LabelOrPointer> = vec![];
        let mut cur = 0;
        let mut cnt = 0;
        let mut label_index = HashMap::new();

        while cnt < qncount {
            loop {
                if bytes[cur] == 0 {
                    tokens.push(LabelOrPointer::Stop);
                    cur += 1;
                    tokens.push(
                        LabelOrPointer::QType(
                            bytes[cur .. cur + 2].to_vec()
                        )
                    );
                    tokens.push(
                        LabelOrPointer::QClass(
                            bytes[cur + 2 .. cur + 4].to_vec()
                        )
                    );
                    cur += 4;
                    break;
                }
                let head = (bytes[cur] >> 6) & 0b11;
                if head == 0b00 {
                    label_index.insert(cur + header_offset, tokens.len());
                    let len = bytes[cur] as usize;
                    tokens.push(
                        LabelOrPointer::Label(
                            bytes[cur..=cur+len].to_vec()
                        )
                    );
                    cur += len + 1;
                }
                else if head == 0b11 {
                    let pointer = (
                        (
                            (bytes[cur] & 0x3f) as u16
                        ) << 8
                    ) 
                    | bytes[cur+1] as u16;
                    tokens.push(
                        LabelOrPointer::Pointer(pointer as usize)
                    );
                    cur += 2;
                    tokens.push(
                        LabelOrPointer::QType(
                            bytes[cur .. cur + 2].to_vec()
                        )
                    );
                    tokens.push(
                        LabelOrPointer::QClass(
                            bytes[cur + 2 .. cur + 4].to_vec()
                        )
                    );
                    cur += 4;
                    break;
                }
                else {
                    panic!("Not a label or pointer");
                }
            }
            cnt += 1;
        }

        // println!("{:#?}", tokens);
        // println!("{:#?}", label_index);

        let mut next = 0;

        let mut questions = vec![];
        for _ in 0..qncount {
            let mut name: Vec<u8> = vec![];
            let mut q_class: Option<Vec<u8>> = None;
            let mut q_type: Option<Vec<u8>> = None;

            let mut jumped = false;
            let mut cursor = next;
            loop {
                match tokens[cursor] {
                    LabelOrPointer::Label(ref label) => {
                        name.extend(label);
                        cursor += 1;
                        if !jumped {
                            next = cursor;
                        }
                    }
                    LabelOrPointer::Pointer(pointer) => {
                        if !jumped {
                            next = cursor + 1;
                            q_type = Some(
                                if let LabelOrPointer::QType(ref q_type) = tokens[next] {
                                    q_type.clone()
                                } else {
                                    panic!("Expect a QType after Pointer, got {:?}", tokens[next]);
                                }
                            );

                            q_class = Some(
                                if let LabelOrPointer::QClass(ref q_class) = tokens[next + 1] {
                                    q_class.clone()
                                } else {
                                    panic!("Expect a QClass after Pointer, got {:?}", tokens[next + 1]);
                                }
                            );
                            next += 2;
                            jumped = true;
                        }
                        cursor = *label_index.get(&pointer).unwrap();
                    }
                    LabelOrPointer::Stop => {
                        next += 1;
                        if !jumped {
                            q_type = Some(
                                if let LabelOrPointer::QType(ref q_type) = tokens[next] {
                                    q_type.clone()
                                } else {
                                    panic!("Expect a QType after Stop, got {:?}", tokens[next]);
                                }
                            );
                            q_class = Some(
                                if let LabelOrPointer::QClass(ref q_class) = tokens[next + 1] {
                                    q_class.clone()
                                } else {
                                    panic!("Expect a QClass after Stop, got {:?}", tokens[next + 1]);
                                }
                            );
                            next += 2;
                        }
                        break;
                    }
                    _ => {
                        panic!("Not a label or pointer");
                    }
                }
            }
            questions.push(Question {
                name,
                qtype: u16::from_be_bytes(q_type.unwrap().try_into().unwrap()),
                qclass: u16::from_be_bytes(q_class.unwrap().try_into().unwrap()),
            });
        }

        (questions, cur)
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