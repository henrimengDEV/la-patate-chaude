use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {
}

// "Hello"
impl Display for Hello {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Hello\"")
    }
}