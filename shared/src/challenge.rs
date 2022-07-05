use std::fmt::{Formatter, Result};
use serde::{Serialize, Deserialize};
use crate::md5_hash_cash_input::MD5HashCashInput;

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    RecoverSecret,
}