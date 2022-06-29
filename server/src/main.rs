use std::io::Write;
use std::net::TcpStream;

fn main() {

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message= "Hello".as_bytes();
            let _response = stream.write_all(&message);
            println!("Hello, Server!");
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}
