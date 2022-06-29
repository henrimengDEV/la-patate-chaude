extern crate shared;

use std::io::Write;
use std::net::TcpStream;
use shared::hello::Hello;
use shared::welcome::Welcome;
use shared::challenge::Challenge;
use shared::challenge_answer::ChallengeAnswer;
use shared::challenge_result::ChallengeResult;
use shared::challenge_value::ChallengeValue;
use shared::end_of_game::EndOfGame;
use shared::public_leader_board::PublicLeaderBoard;
use shared::public_player::PublicPlayer;
use shared::reported_challenge_result::ReportedChallengeResult;
use shared::round_summary::RoundSummary;
use shared::subscribe::Subscribe;
use shared::subscribe_error::SubscribeError;
use shared::subscribe_result::SubscribeResult;


fn main() {
    let stream = TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...");
    let message = "\"Hello\"";
    let _result = request_server(stream, message);
    // stream.read((_result.unwrap() as u32).to_be_bytes());

    let hello = Hello {};
    println!("{}", hello);

    let welcome = Welcome { version: 1 };
    println!("{}", welcome);

    // TODO

    let challenge = Challenge {};
    println!("{}", challenge);

    let challenge_answer = ChallengeAnswer {};
    println!("{}", challenge_answer);

    let challenge_result = ChallengeResult {};
    println!("{}", challenge_result);

    let challenge_value = ChallengeValue {};
    println!("{}", challenge_value);

    let end_of_game = EndOfGame {};
    println!("{}", end_of_game);

    let public_leader_board = PublicLeaderBoard {};
    println!("{}", public_leader_board);

    let public_player = PublicPlayer {};
    println!("{}", public_player);

    let reported_challenge_result = ReportedChallengeResult {};
    println!("{}", reported_challenge_result);

    let round_summary = RoundSummary {};
    println!("{}", round_summary);

    let subscribe = Subscribe {};
    println!("{}", subscribe);

    let subscribe_error = SubscribeError {};
    println!("{}", subscribe_error);

    let subscribe_result = SubscribeResult {};
    println!("{}", subscribe_result);

}

fn request_server(
    mut stream: TcpStream,
    message: &str,
) -> Result<usize, std::io::Error> {
    let message_len = &(message.len() as u32).to_be_bytes();
    let _result = stream.write(message_len);
    return stream.write(message.as_bytes());
}