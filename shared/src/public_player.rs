use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

// {"name":"free_patato","stream_id":"127.0.0.1","score":10,"steps":20,"is_active":true,"total_used_time":1.234}
impl Display for PublicPlayer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\"name\":\"{}\",\
        \"stream_id\":\"{}\",\
        \"score\":10,\"steps\":{},\
        \"is_active\":{},\
        \"total_used_time\":{}}}",
               self.name, self.stream_id, self.score, self.is_active, self.total_used_time
        )
    }
}