use std::fmt::{Display, Formatter, Result};

pub struct ChallengeValue {
}

// "Hello"
impl Display for ChallengeValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Hello\"")
    }
}