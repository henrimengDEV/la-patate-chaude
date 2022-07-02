extern crate shared;

use std::io::Write;
use std::net::TcpStream;
use shared::challenge::{Challenge, MD5HashCash};
use shared::hello::Hello;
use shared::public_leader_board::PublicLeaderBoard;
use shared::public_player::PublicPlayer;
use shared::subscribe::Subscribe;
use shared::subscribe_error::SubscribeError;
use shared::subscribe_result::SubscribeResult;
use shared::welcome::Welcome;

fn main() {
    // let stream = TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...");
    // let message = "\"Hello\"";
    // let _result = request_server(stream, message);
    // stream.read((_result.unwrap() as u32).to_be_bytes());

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
            }
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

fn request_server(
    mut stream: TcpStream,
    message: &str,
) -> Result<usize, std::io::Error> {
    let message_len = &(message.len() as u32).to_be_bytes();
    let _result = stream.write(message_len);
    return stream.write(message.as_bytes());
}