use crate::message::header::Header;

pub struct Message {
    pub header: Header
}

impl Message {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.encode());
        bytes
    }
}