use std::fmt::{Display, Formatter, Result};

pub struct PublicPlayer {
}

// Additionnal Type
// name: String
// stream_id: String
// score: i32
// steps: u32
// is_active: bool
// total_used_time: f64

impl Display for PublicPlayer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Public Player\"")
    }
}