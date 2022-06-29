use std::fmt::{Display, Formatter, Result};

pub struct Subscribe {
}

// {"SubscribeResult":{"Err":"InvalidName"}}
impl Display for Subscribe {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Subscribe\"")
    }
}