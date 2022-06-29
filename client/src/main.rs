use std::io::Write;
use std::net::TcpStream;

use crate::hello::Hello;
use crate::welcome::Welcome;

mod hello;
mod welcome;

fn main() {
    let stream = TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...");
    let message = "\"Hello\"";
    let _result = request_server(stream, message);
    // stream.read((_result.unwrap() as u32).to_be_bytes());

    let hello = Hello {};
    println!("{}", hello);

    let welcome = Welcome { version: 1 };
    println!("{}", welcome);
}

fn request_server(
    mut stream: TcpStream,
    message: &str,
) -> Result<usize, std::io::Error> {
    let message_len = &(message.len() as u32).to_be_bytes();
    let _result = stream.write(message_len);
    return stream.write(message.as_bytes());
}