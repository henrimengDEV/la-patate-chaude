use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::public_player::PublicPlayer;

#[derive(Serialize, Deserialize)]
pub struct PublicLeaderBoard {
    pub public_players: Vec<PublicPlayer>,
}

// {"PublicLeaderBoard":[
// {"name":"free_patato","stream_id":"127.0.0.1","score":10,"steps":20,"is_active":true,"total_used_time":1.234},
// {"name":"dark_salad","stream_id":"127.0.0.1","score":6,"steps":200,"is_active":true,"total_used_time":0.1234}
// ]}

// Additionnal Type
//.0: Vec<PublicPlayer>

impl Display for PublicLeaderBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let public_players = self.public_players.clone()
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{{\"PublicLeaderBoard\":[{}]}}", public_players)
    }
}