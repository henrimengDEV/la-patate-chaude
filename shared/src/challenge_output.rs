use std::fmt::{Formatter, Result};
use serde::{Serialize, Deserialize};
use crate::md5_hash_cash_output::MD5HashCashOutput;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeOutput {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret,
}