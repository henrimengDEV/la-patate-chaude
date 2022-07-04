use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

impl Display for SubscribeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubscribeError::AlreadyRegistered => write!(f, "\"AlreadyRegistered\""),
            SubscribeError::InvalidName => write!(f, "\"InvalidName\""),
        }
    }
}