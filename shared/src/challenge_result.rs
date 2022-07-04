use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ChallengeResult {
}

// {"ChallengeResult":{"answer":{"MD5HashCash":{"seed":12345678,"hashcode":"68B329DA9893E34099C7D8AD5CB9C940"}},"next_target":"dark_salad"}}

// type
//name: ChallengeAnswer
// next_target: String

impl Display for ChallengeResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Challenge Result\"")
    }
}