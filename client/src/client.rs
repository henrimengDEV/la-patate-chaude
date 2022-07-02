use std::io::Write;
use std::net::TcpStream;
use crate::message_type::MessageType;

pub struct Client {
    pub stream: TcpStream
}

impl Client {
    pub fn send(&mut self, message_type: MessageType) {
        println!("{}", message_type.to_string());
        // let msg = message_type.to_string();
        //
        // println!("send");
        // println!("{}", msg);
        //
        // let msg_len = &(msg.len() as u32).to_be_bytes();
        // self.stream.write(msg_len).unwrap();
        // self.stream.write(msg.as_bytes()).unwrap();
    }
}