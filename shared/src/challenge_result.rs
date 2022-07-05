
use serde::{Serialize, Deserialize};
use crate::challenge_answer::ChallengeAnswer;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}