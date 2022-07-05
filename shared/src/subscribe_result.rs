use std::fmt::{Formatter, Result};
use crate::subscribe_error::SubscribeError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum  SubscribeResult {
    Ok,
    Err(SubscribeError)
}