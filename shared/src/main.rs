use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("Hello, Shared!");

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message = "Hello".as_bytes();
            let _response = stream.write_all(&message);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}
