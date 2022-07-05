
use serde::{Serialize, Deserialize};
use crate::reported_challenge_result::ReportedChallengeResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
}