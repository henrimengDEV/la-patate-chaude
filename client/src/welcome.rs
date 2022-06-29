use std::fmt::{Display, Formatter, Result};

pub struct Welcome {
    pub version: u8
}

// {"Welcome":{"version":1}}
impl Display for Welcome {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"Welcome\":{{\"version\":{}}}}}", self.version)
    }
}

