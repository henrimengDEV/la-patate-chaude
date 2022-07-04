use std::fmt::{Display, Formatter, Result};
use crate::subscribe_error::SubscribeError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum  SubscribeResult {
    Ok,
    Err(SubscribeError)
}

// {"SubscribeResult":{"err":"InvalidName"}}
impl Display for SubscribeResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubscribeResult::Ok => write!(f, "{{\"SubscribeResult\":{{\"Ok\"}}}}"),
            SubscribeResult::Err(it) => write!(f, "{{\"SubscribeResult\":{{\"Err\":{}}}}}", it)
        }
    }
}