use std::fmt::{Display, Formatter, Result};

pub struct Challenge {
    pub md5_hash_cash: MD5HashCash,
}

// {"Challenge":{"MD5HashCash":{"complexity":5,"message":"Hello"}}}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"Challenge\"{}}}", self.md5_hash_cash)
    }
}

pub struct MD5HashCash {
    pub complexity: u8,
    pub message: String,
}

impl Display for MD5HashCash {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"MD5HashCash\":{{\"complexity\":{}, \"message\":\"{}\"}}}}", self.complexity, self.message)
    }
}