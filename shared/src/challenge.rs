use std::fmt::{Display, Formatter, Result};

pub struct Challenge {
}

// {"Challenge":{"MD5HashCash":{"complexity":5,"message":"Hello"}}}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Challenge\"")
    }
}