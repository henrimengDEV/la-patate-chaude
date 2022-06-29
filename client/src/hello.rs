use std::fmt::{Display, Formatter, Result};

pub struct Hello {
}

// "Hello"
impl Display for Hello {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Hello\"")
    }
}