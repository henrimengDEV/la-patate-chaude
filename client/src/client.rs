use std::char::from_u32;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use crate::message_type::MessageType;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn send(&mut self, message_type: MessageType) {
        let msg = message_type.to_string();
        let msg_len = &(msg.len() as u32).to_be_bytes();
        self.stream.write(msg_len).unwrap();
        self.stream.write(msg.as_bytes()).unwrap();
        println!("\t> Sent {}", msg);

        let mut buffer = [0 as u8; 4];
        match self.stream.read_exact(&mut buffer) {
            Ok(_) => {
                let message_size = u32::from_be_bytes(buffer);
                println!("\t> Reply message size is: {}", message_size);

                let mut message_content = vec![0u8; message_size as usize];
                match self.stream.read_exact(&mut message_content) {
                    Ok(_) => {
                        let response = from_utf8(&message_content).unwrap();
                        println!("\t> Reply message is: {}", response);
                    }
                    Err(e) => {
                        println!("\t> Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("\t> Failed to receive data: {}", e);
            }
        }
    }
}