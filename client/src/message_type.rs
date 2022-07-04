use std::fmt::{Display, Formatter, Result};
use shared::challenge::Challenge;
use shared::challenge_result::ChallengeResult;
use shared::end_of_game::EndOfGame;
use shared::hello::Hello;
use shared::public_leader_board::PublicLeaderBoard;
use shared::round_summary::RoundSummary;
use shared::subscribe::Subscribe;
use shared::subscribe_result::SubscribeResult;
use shared::welcome::Welcome;

pub enum MessageType {
    Hello { value: Hello },
    Welcome { value: Welcome },
    Subscribe { value: Subscribe },
    SubscribeResult { value: SubscribeResult },
    PublicLeaderBoard { value: PublicLeaderBoard },
    Challenge { value: Challenge },
    ChallengeResult { value: ChallengeResult },
    RoundSummary { value: RoundSummary },
    EndOfGame { value: EndOfGame },
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MessageType::Hello { value } => write!(f, "{}", value),
            MessageType::Welcome { value } => write!(f, "{}", value),
            MessageType::Subscribe { value } => write!(f, "{}", value),
            MessageType::SubscribeResult { value } => write!(f, "{}", value),
            MessageType::PublicLeaderBoard { value } => write!(f, "{}", value),
            MessageType::Challenge { value } => write!(f, "{}", value),
            MessageType::ChallengeResult { value } => write!(f, "{}", value),
            MessageType::RoundSummary { value } => write!(f, "{}", value),
            MessageType::EndOfGame { value } => write!(f, "{}", value)
        }
    }
}
