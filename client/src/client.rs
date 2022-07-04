use std::char::from_u32;
use std::fmt::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::str::from_utf8;
use shared::welcome::Welcome;
use crate::message_type::MessageType;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn send(&mut self, message_type: MessageType) {
        let msg = message_type.to_string();
        let msg_len = &(msg.len() as u32).to_be_bytes();
        self.stream.write(msg_len).expect("Failed to send message size !");
        self.stream.write(msg.as_bytes()).expect("Failed to send message content !");

        println!("\t> Sent {}", msg);

        let message_size = self.read_message_size();
        self.read_message_content(message_size as usize);
    }

    fn read_message_size(&mut self) -> u32 {
        let mut buffer = [0 as u8; 4];
        self.stream.read_exact(&mut buffer).expect("Failed to read message size !");
        let message_size = u32::from_be_bytes(buffer);
        println!("\t> Reply message size is: {}", message_size);
        message_size
    }

    fn read_message_content(&mut self, size: usize) {
        let mut message_content = vec![0u8; size];
        self.stream.read_exact(&mut message_content).expect("Failed to read message content !");
        let response = from_utf8(&message_content).unwrap();
        let deserialized: MessageType = serde_json::from_str(&response).unwrap();
        println!("\t> Reply message is: {}", &deserialized);

    }
}