use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

// {"Subscribe":{"name":"free_patato"}}
impl Display for Subscribe {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"Subscribe\":{{\"name\":\"{}\"}}}}", self.name)
    }
}