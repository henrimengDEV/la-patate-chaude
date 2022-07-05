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