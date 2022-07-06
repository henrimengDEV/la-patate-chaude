
use serde::{Serialize, Deserialize};
use crate::public_player::PublicPlayer;

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    pub leader_board: Vec<PublicPlayer>
}