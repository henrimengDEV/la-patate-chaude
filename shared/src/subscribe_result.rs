use std::fmt::{Display, Formatter, Result};
use crate::subscribe_error::SubscribeError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SubscribeResult {
    pub err: SubscribeError
}

// {"SubscribeResult":{"err":"InvalidName"}}
impl Display for SubscribeResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"SubscribeResult\":{{\"Err\":{}}}}}", self.err)
    }
}