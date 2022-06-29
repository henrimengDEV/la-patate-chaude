use std::fmt::{Display, Formatter, Result};

pub struct SubscribeResult {
}

// {"SubscribeResult":{"Err":"InvalidName"}}
impl Display for SubscribeResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Subscribe Result\"")
    }
}