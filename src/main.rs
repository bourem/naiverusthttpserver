use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to start server");

    fn read_request(stream: &TcpStream) -> String {
        let mut istream = stream.try_clone().expect("couldn't clone");
        let reader = io::BufReader::new(stream);
        let mut request = String::new();
        let mut l;
        for line in reader.lines() {
            l = line.unwrap();
            if l == "" {
                istream.flush().expect("stream couldn't be flushed");
                break;
            } else {
                request.push_str(&l);
                request.push_str("\n");
            }
        }
        println!("finished reading");
        request
    }

    fn handle_client(mut stream: TcpStream) {
        println!("Hello, world!");

        let request;

        request = read_request(&stream);
        println!("{:?} - {:?}", request.len(), request);
        
        stream.write_all(b"HTTP/1.0 200 OK\nContent-Type: text/html\n\n<html><body>Hello there!</body></html>").expect("write failed");

        stream.flush().expect("stream couldn't be flushed");
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
