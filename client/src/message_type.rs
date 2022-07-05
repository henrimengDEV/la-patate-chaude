use std::fmt::{Formatter, Result};
use serde::{Serialize, Deserialize};
use shared::challenge::Challenge;
use shared::challenge_result::ChallengeResult;
use shared::end_of_game::EndOfGame;
use shared::public_leader_board::PublicLeaderBoard;
use shared::public_player::PublicPlayer;
use shared::round_summary::RoundSummary;
use shared::subscribe::Subscribe;
use shared::subscribe_result::SubscribeResult;
use shared::welcome::Welcome;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

// impl Display for MessageType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             MessageType::Hello(it) => write!(f, "{}", it),
//             MessageType::Welcome(it) => write!(f, "{}", it),
//             MessageType::Subscribe(it) => write!(f, "{}", it),
//             MessageType::SubscribeResult(it) => write!(f, "{}", it),
//             MessageType::PublicLeaderBoard(it) => {
//                 let public_players = it
//                     .into_iter()
//                     .map(|i| i.to_string())
//                     .collect::<Vec<String>>()
//                     .join(",");
//                 write!(f, "{{\"PublicLeaderBoard\":[{}]}}", public_players)
//             },
//             MessageType::Challenge(it) => write!(f, "{}", it),
//             MessageType::ChallengeResult(it) => write!(f, "{}", it),
//             MessageType::RoundSummary(it) => write!(f, "{}", it),
//             MessageType::EndOfGame(it) => write!(f, "{}", it)
//         }
//     }
// }
