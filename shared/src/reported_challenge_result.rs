use std::fmt::{Display, Formatter, Result};

pub struct ReportedChallengeResult {
}

// Additionnal Type
// "name: String,
// value: JobValue"
impl Display for ReportedChallengeResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Reported Challenge Result\"")
    }
}