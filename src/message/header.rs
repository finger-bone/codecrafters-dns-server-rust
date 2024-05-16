use anyhow::{Result, anyhow};

pub struct Header {
    // 16 bits
    pub id: u16,
    pub qr: bool,
    // 4 bits
    pub opcode: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    // 3 bits
    pub z: u8,
    // 4 bits
    pub rcode: u8,
    // 16 bits
    pub qdcount: u16,
    // 16 bits
    pub ancount: u16,
    // 16 bits
    pub nscount: u16,
    // 16 bits
    pub arcount: u16
}

impl Header {
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::new()
    }

    pub fn encode(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(((self.id >> 8) & 0xFF) as u8);
        bytes.push((self.id & 0xFF) as u8);
        let flags = (self.qr as u8) << 7 | (self.opcode << 3) | (self.aa as u8) << 2 | (self.tc as u8) << 1 | self.rd as u8;
        bytes.push(flags);
        let flags = (self.ra as u8) << 7 | (self.z << 4) | self.rcode;
        bytes.push(flags);
        bytes.push(((self.qdcount >> 8) & 0xFF) as u8);
        bytes.push((self.qdcount & 0xFF) as u8);
        bytes.push(((self.ancount >> 8) & 0xFF) as u8);
        bytes.push((self.ancount & 0xFF) as u8);
        bytes.push(((self.nscount >> 8) & 0xFF) as u8);
        bytes.push((self.nscount & 0xFF) as u8);
        bytes.push(((self.arcount >> 8) & 0xFF) as u8);
        bytes.push((self.arcount & 0xFF) as u8);
        bytes
    }
}
pub struct HeaderBuilder {
    id: Option<u16>,
    qr: Option<bool>,
    opcode: Option<u8>,
    aa: Option<bool>,
    tc: Option<bool>,
    rd: Option<bool>,
    ra: Option<bool>,
    z: Option<u8>,
    rcode: Option<u8>,
    qdcount: Option<u16>,
    ancount: Option<u16>,
    nscount: Option<u16>,
    arcount: Option<u16>
}

#[allow(dead_code)]
impl HeaderBuilder {

    pub fn new() -> HeaderBuilder {
        HeaderBuilder {
            id: None,
            qr: None,
            opcode: None,
            aa: None,
            tc: None,
            rd: None,
            ra: None,
            z: None,
            rcode: None,
            qdcount: None,
            ancount: None,
            nscount: None,
            arcount: None
        }
    }

    pub fn id(mut self, id: u16) -> Result<HeaderBuilder> {
        self.id = Some(id);
        Ok(self)
    }

    pub fn qr(mut self, qr: bool) -> Result<HeaderBuilder> {
        self.qr = Some(qr);
        Ok(self)
    }

    pub fn opcode(mut self, opcode: u8) -> Result<HeaderBuilder> {
        if opcode > (1 << 4) {
            return Err(anyhow!("OPCODE must be a 4-bit number"));
        }
        self.opcode = Some(opcode);
        Ok(self)
    }

    pub fn aa(mut self, aa: bool) -> Result<HeaderBuilder> {
        self.aa = Some(aa);
        Ok(self)
    }

    pub fn tc(mut self, tc: bool) -> Result<HeaderBuilder> {
        self.tc = Some(tc);
        Ok(self)
    }

    pub fn rd(mut self, rd: bool) -> Result<HeaderBuilder> {
        self.rd = Some(rd);
        Ok(self)
    }

    pub fn ra(mut self, ra: bool) -> Result<HeaderBuilder> {
        self.ra = Some(ra);
        Ok(self)
    }

    pub fn z(mut self, z: u8) -> Result<HeaderBuilder> {
        if z > (1 << 3) {
            return Err(anyhow!("Z must be a 3-bit number"));
        }
        self.z = Some(z);
        Ok(self)
    }

    pub fn rcode(mut self, rcode: u8) -> Result<HeaderBuilder> {
        if rcode > (1 << 4) {
            return Err(anyhow!("RCODE must be a 4-bit number"));
        }
        self.rcode = Some(rcode);
        Ok(self)
    }

    pub fn qdcount(mut self, qdcount: u16) -> Result<HeaderBuilder> {
        self.qdcount = Some(qdcount);
        Ok(self)
    }

    pub fn ancount(mut self, ancount: u16) -> Result<HeaderBuilder> {
        self.ancount = Some(ancount);
        Ok(self)
    }

    pub fn nscount(mut self, nscount: u16) -> Result<HeaderBuilder> {
        self.nscount = Some(nscount);
        Ok(self)
    }

    pub fn arcount(mut self, arcount: u16) -> Result<HeaderBuilder> {
        self.arcount = Some(arcount);
        Ok(self)
    }

    pub fn build(self) -> Header {
        Header {
            id: self.id.unwrap_or(1234),
            qr: self.qr.unwrap_or(true),
            opcode: self.opcode.unwrap_or(0),
            aa: self.aa.unwrap_or(false),
            tc: self.tc.unwrap_or(false),
            rd: self.rd.unwrap_or(false),
            ra: self.ra.unwrap_or(false),
            z: self.z.unwrap_or(0),
            rcode: self.rcode.unwrap_or(0),
            qdcount: self.qdcount.unwrap_or(0),
            ancount: self.ancount.unwrap_or(0),
            nscount: self.nscount.unwrap_or(0),
            arcount: self.arcount.unwrap_or(0)
        }
    }


}