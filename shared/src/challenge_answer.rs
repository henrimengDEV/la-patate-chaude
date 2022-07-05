
use serde::{Serialize, Deserialize};
use crate::md5_hash_cash_output::MD5HashCashOutput;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    // RecoverSecret,
}