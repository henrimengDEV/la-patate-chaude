use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message= "Hello".as_bytes();
            let response = stream.write_all(&message);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}