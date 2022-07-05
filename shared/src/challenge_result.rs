use std::fmt::{Formatter, Result};
use serde::{Serialize, Deserialize};
use crate::challenge_answer::ChallengeAnswer;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    pub name: ChallengeAnswer,
    pub next_target: String
}