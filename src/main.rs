use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::io;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to start server");

    fn handle_client(stream: TcpStream) {
        println!("Hello, world!");

        let mut ostream = stream.try_clone().expect("couldn't clone");
        let mut reader = io::BufReader::new(stream);
        let mut request = String::new();

        let a = reader.read_line(&mut request).expect("read failed");
        println!("{:?} - {:?}", a, request);
        
        ostream.write_all(b"HTTP/1.0 200 OK\nContent-Type: text/html\n\n<html><body>Hello there!</body></html>").expect("write failed");

        ostream.flush().expect("stream flushed");
        match ostream.shutdown(Shutdown::Both) {
            Ok(_) => println!("shutdown call successful"),
            Err(e) => println!("shutdown call failed, with error: {:?}", e)
        };
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
