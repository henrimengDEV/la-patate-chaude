use std::fmt::{Display, Formatter, Result};

pub struct PublicLeaderBoard {
}

// {"PublicLeaderBoard":[
// {"name":"free_patato","stream_id":"127.0.0.1","score":10,"steps":20,"is_active":true,"total_used_time":1.234},
// {"name":"dark_salad","stream_id":"127.0.0.1","score":6,"steps":200,"is_active":true,"total_used_time":0.1234}
// ]}

// Additionnal Type
//.0: Vec<PublicPlayer>

impl Display for PublicLeaderBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\"Public Leader Board\"")
    }
}