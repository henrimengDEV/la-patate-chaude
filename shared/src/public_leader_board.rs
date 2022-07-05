
use serde::{Deserialize, Serialize};
use crate::public_player::PublicPlayer;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard {
    pub public_players: Vec<PublicPlayer>,
}