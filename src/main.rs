use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    fn handle_client(mut stream: TcpStream) {
        println!("Hello, world!");
        let mut request = [0;10];
        let a = stream.read(&mut request);
        println!("{:?} - {:?}", a, request);
        let b = stream.write(b"HTTP/1.0 200 OK\nContent-Type: text/html\n\n<html><body>Hello there!</body></html>");
        println!("{:?}", b);
        stream.flush();
        stream.shutdown(Shutdown::Both);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
