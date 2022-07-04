mod client;
mod message_type;
mod hash_cash;

extern crate shared;

use std::net::TcpStream;
use shared::hello::Hello;
use shared::md5_hash_cash_input::MD5HashCashInput;
use crate::client::Client;
use crate::hash_cash::HashCash;
use crate::message_type::MessageType;

fn main() {
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