use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use crate::message_type::MessageType;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn watching(&mut self) -> MessageType {
        println!("\n\t> I'm watching ...");
        let message_size = self.read_message_size();
        self.read_message_content(message_size as usize)
    }

    pub fn send(&mut self, message_type: MessageType) {
        let msg = serde_json::to_string(&message_type).expect(format!("Failed to serialize {:?}", message_type).as_str());
        let msg_len = &(msg.len() as u32).to_be_bytes();
        self.stream.write(msg_len).expect("Failed to send message size !");
        self.stream.write(msg.as_bytes()).expect("Failed to send message content !");

        println!("\n\t> Sent {:?}", msg);
    }

     fn read_message_size(&mut self) -> u32 {
        let mut buffer = [0 as u8; 4];
        self.stream.read_exact(&mut buffer).expect("Failed to read message size !");
        let message_size = u32::from_be_bytes(buffer);
        println!("\t> Reply message size is: {}", message_size);
        message_size
    }

     fn read_message_content(&mut self, size: usize) -> MessageType {
        let mut message_content = vec![0u8; size];
        self.stream.read_exact(&mut message_content).expect("Failed to read message content !");
        let response = from_utf8(&message_content).unwrap();
        let deserialized: MessageType = serde_json::from_str(&response).unwrap();
        println!("\t> Reply message is: {:?}", &deserialized);
        deserialized
    }
}



#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::message_type::MessageType;
    use std::net::TcpStream;

    #[test]
    fn test_watching_hello() {
        let mut client = Client {
            stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
        };
        let test = MessageType::Hello;
        client.send(test);
        let result = Client::watching(&mut client);
        assert!(matches!(result, test));
    }

    #[test]
    fn test_read_message_size() {
        let mut client = Client {
            stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
        };
        let test = MessageType::Hello;
        client.send(test);

        let result = Client::read_message_size(&mut client);
        assert_eq!(result, 25);
    }

    // #[test]
    // fn test_read_message_content() {
    //     let mut client = Client {
    //         stream: TcpStream::connect("localhost:7878").expect("Couldn't connect to the server...")
    //     };
    //
    //     let test = MessageType::Hello;
    //     let result = Client::read_message_content(&mut client, 2);
    //     assert!(matches!(result, test))
    // }


}