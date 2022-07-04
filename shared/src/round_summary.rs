use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RoundSummary {
}

// {"RoundSummary":{"challenge":"MD5HashCash","chain":[
// {"name":"free_patato","value":{"Ok":{"used_time":0.1,"next_target":"dark_salad"}}},
// {"name":"dark_salad","value":"Unreachable"}
// ]}}

impl Display for RoundSummary {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Round Summary\"")
    }
}