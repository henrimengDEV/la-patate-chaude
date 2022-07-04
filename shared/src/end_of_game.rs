use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EndOfGame {}

// {"EndOfGame":{"leader_board":[
// {"name":"free_patato","stream_id":"127.0.0.1","score":10,"steps":20,"is_active":true,"total_used_time":1.234},
// {"name":"dark_salad","stream_id":"127.0.0.1","score":6,"steps":200,"is_active":true,"total_used_time":0.1234}
// ]}}
impl Display for EndOfGame {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"End Of Game\"")
    }
}