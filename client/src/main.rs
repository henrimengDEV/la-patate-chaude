mod client;
mod message_type;
mod hash_cash;

extern crate shared;

use std::io::Write;
use std::net::TcpStream;
use shared::challenge::{Challenge, MD5HashCash};
use shared::hello::Hello;
use shared::md5_hash_cash_input::MD5HashCashInput;
use shared::public_leader_board::PublicLeaderBoard;
use shared::public_player::PublicPlayer;
use shared::subscribe::Subscribe;
use shared::subscribe_error::SubscribeError;
use shared::subscribe_result::SubscribeResult;
use shared::welcome::Welcome;
use crate::client::Client;
use crate::hash_cash::HashCash;
use crate::message_type::MessageType;

fn main() {
    // let serialized = serde_json::to_string(&Welcome { version: 1 }).unwrap();
    // println!("{}", &serialized);
    // let deserialized: MessageType = serde_json::from_str(&serialized).unwrap();
    // println!("{}", &deserialized);

    println!("Communication Started with localhost:7878 ...");
    let mut client = Client {
        stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
    };
    client.send(MessageType::Hello(Hello {}));
    println!("Communication Terminated.");


    let mut hash_cash = HashCash::new(
        MD5HashCashInput { complexity: 9, message: String::from("hello") }
    );
    hash_cash.run();
}

fn test_model() {
    let hello = Hello {};
    println!("{}", hello);

    let welcome = Welcome { version: 1 };
    println!("{}", welcome);

    let subscribe = Subscribe { name: String::from("free_potato") };
    println!("{}", subscribe);

    let subscribe_result = SubscribeResult {
        err: SubscribeError::AlreadyRegistered
    };
    println!("{}", subscribe_result);

    let public_leader_board = PublicLeaderBoard {
        public_players: vec![
            PublicPlayer {
                name: String::from("free_patato"),
                stream_id: String::from("free_patato"),
                score: 10,
                steps: 20,
                is_active: true,
                total_used_time: 1.234,
            },
            PublicPlayer {
                name: String::from("free_patato"),
                stream_id: String::from("free_patato"),
                score: 10,
                steps: 20,
                is_active: true,
                total_used_time: 1.234,
            },
        ]
    };
    println!("{}", public_leader_board);

    let challenge = Challenge {
        md5_hash_cash: MD5HashCash {
            complexity: 5,
            message: String::from("Hello"),
        }
    };
    println!("{}", challenge);
}