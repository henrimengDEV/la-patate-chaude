use std::fmt::{Display, Formatter, Result};

pub struct SubscribeError {
}

// enum { AlreadyRegistered, InvalidName }
impl Display for SubscribeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Subscribe Error\"")
    }
}