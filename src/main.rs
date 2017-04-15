use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to start server");

    #[derive(Debug)]
    enum HTTPMethod {
        POST,
        GET,
        PUT,
        HEAD,
        OPTIONS,
        DELETE,
        NONE,
    }

    #[derive(Debug)]
    struct Request {
        method: HTTPMethod,
        resource: String,
        http_version: String,
        content_length: u64,
        content: String,
    }

    fn read_request(stream: &TcpStream) -> Request {
        let istream = stream.try_clone().expect("couldn't clone");
        let reader = io::BufReader::new(istream);
        let mut request_string = String::new();
        let mut l;

        let mut request = Request { 
            method: HTTPMethod::NONE,
            resource: String::new(),
            http_version: String::new(),
            content_length: 0,
            content: String::new(),
        };

        for (i, line) in reader.lines().enumerate() {
            l = line.unwrap();
            
            if i == 0 {
                let mut iter = l.split_whitespace();
                request.method = match iter.next().unwrap() {
                    "GET" => HTTPMethod::GET,
                    "POST" => HTTPMethod::POST,
                    "PUT" => HTTPMethod::PUT,
                    "HEAD" => HTTPMethod::HEAD,
                    "OPTIONS" => HTTPMethod::OPTIONS,
                    "DELETE" => HTTPMethod::DELETE,
                    &_ => HTTPMethod::NONE,
                };
                request.resource = iter.next().unwrap().to_string();
                request.http_version = iter.next().unwrap().to_string();
            }
            match l.as_str() {
                "" => {
                    break;
                },
                a => {
                    let mut iter = a.split_whitespace();
                    match iter.next().unwrap() {
                        "Content-Length:" => {
                            let cl = iter.next().unwrap();
                            request.content_length = cl.parse().unwrap();
                        },
                        _ => {
                            request_string.push_str(&l);
                            request_string.push_str("\n");
                        },
                    }
                },
            }
        }
        if request.content_length > 0 {
            let mut buf = vec![];
            let content_stream = stream.try_clone().expect("");
            let mut handle = content_stream.take(request.content_length);
            println!("{}", request.content_length);
            handle.read(&mut buf).unwrap();
            println!("Content: {:?}", buf);
        }
        let mut stream = stream;
        stream.flush().expect("stream couldn't be flushed");
        println!("finished reading");
        println!("{:?}", request_string);
        request
    }

    fn handle_client(mut stream: TcpStream) {
        println!("Hello, world!");

        let request;

        request = read_request(&stream);
        println!("{:?}", request);
        
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
