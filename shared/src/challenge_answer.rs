use std::fmt::{Display, Formatter, Result};

pub struct ChallengeAnswer {
}

// enum { ChallengeName(ChallengeOutput) }
impl Display for ChallengeAnswer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Challenge Answer\"")
    }
}