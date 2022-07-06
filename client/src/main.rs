mod client;
mod message_type;
mod hash_cash;
mod args;

extern crate shared;

use clap::Parser;
use std::net::TcpStream;
use rand::{Rng, thread_rng};
use shared::challenge::Challenge;
use shared::challenge_answer::ChallengeAnswer;
use shared::challenge_result::ChallengeResult;
use shared::public_player::PublicPlayer;
use shared::subscribe::Subscribe;
use crate::args::Arguments;
use crate::client::Client;
use crate::hash_cash::HashCash;
use crate::message_type::MessageType;

fn main() {
    let arguments: Arguments = Arguments::parse();
    println!("Hello {}!", &arguments.name);

    println!("Communication Started with localhost:7878 ...");
    let mut client = Client {
        stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
    };

    client.send(MessageType::Hello);
    loop {
        let message_type = client.watching();
        let mut public_player: Vec<PublicPlayer> = vec!();
        match message_type {
            MessageType::Welcome(_) => {
                client.send(MessageType::Subscribe(Subscribe { name: arguments.clone().name }));
            }
            MessageType::Challenge(it) => {
                let challenge_answer = match it {
                    Challenge::MD5HashCash(it) => {
                        let mut hash_cash = HashCash::new(it);
                        hash_cash.run(false);
                        ChallengeAnswer::MD5HashCash(hash_cash.output)
                    }
                };
                let mut rng = thread_rng();
                client.send(MessageType::ChallengeResult(
                    ChallengeResult {
                        answer: challenge_answer,
                        // next_target: public_player[rng.gen_range(0..public_player.len() - 1)].clone().name,
                        next_target: String::from(""),
                    }
                ));
            }
            MessageType::PublicLeaderBoard(it) => {
                public_player = it;
            }
            MessageType::EndOfGame(_) => {
                break;
            }
            _ => {}
        }
    }

    println!("Communication Terminated.");
}