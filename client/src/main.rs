mod client;
mod message_type;
mod hash_cash;

extern crate shared;

use std::net::TcpStream;
use shared::challenge::Challenge;
use shared::challenge_answer::ChallengeAnswer;
use shared::challenge_output::ChallengeOutput;
use shared::challenge_result::ChallengeResult;
use shared::md5_hash_cash_input::MD5HashCashInput;
use shared::md5_hash_cash_output::MD5HashCashOutput;
use shared::subscribe::Subscribe;
use crate::client::Client;
use crate::hash_cash::HashCash;
use crate::message_type::MessageType;

fn main() {
    println!("Communication Started with localhost:7878 ...");
    let mut client = Client {
        stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
    };

    client.send(MessageType::Hello);
    loop {
        let message_type = client.watching();
        match message_type {
            MessageType::Welcome(_) => {
                client.send(MessageType::Subscribe(Subscribe { name: String::from("Henri") }));
            }
            MessageType::SubscribeResult(_) => {}
            MessageType::PublicLeaderBoard(_) => {}
            MessageType::Challenge(challenge) => {
                let challenge_answer = match challenge {
                    Challenge::MD5HashCash(it) => {
                        let mut hash_cash = HashCash::new(it);
                        hash_cash.run();
                        ChallengeAnswer::ChallengeName(ChallengeOutput::MD5HashCash(hash_cash.output))
                    }
                };
                client.send(MessageType::ChallengeResult(
                    ChallengeResult {
                        name: challenge_answer,
                        next_target: String::from(""),
                    }
                ));
            }
            MessageType::RoundSummary(_) => {}
            MessageType::EndOfGame(_) => {
                break;
            }
            _ => { panic!("Not handled yet") }
        }
    }

    println!("Communication Terminated.");
}