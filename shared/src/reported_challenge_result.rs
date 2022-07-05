use serde::{Serialize, Deserialize};

use crate::challenge_value::ChallengeValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue,
}