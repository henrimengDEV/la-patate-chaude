use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use crate::message_type::MessageType;

pub struct Client {
    pub stream: TcpStream
}

impl Client {
    pub fn send(&mut self, message_type: MessageType) {
        let msg = message_type.to_string();
        let msg_len = &(msg.len() as u32).to_be_bytes();
        self.stream.write(msg_len).unwrap();
        self.stream.write(msg.as_bytes()).unwrap();
        println!("\t> Sent {}", msg);

        let mut data = [0 as u8; 4];
        match self.stream.read_exact(&mut data) {
            Ok(_) => {
                if &data == msg.as_bytes() {
                    println!("\t> Reply is ok!");
                } else {
                    let text = from_utf8(&data).unwrap();
                    println!("\t> Unexpected reply: {}", text);
                }
            },
            Err(e) => {
                println!("\t> Failed to receive data: {}", e);
            }
        }
    }
}