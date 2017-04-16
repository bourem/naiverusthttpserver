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
        Other(String),
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
        let mut reader = io::BufReader::new(stream);
        let mut request_string = String::new();
        let mut l: String;

        let mut request = Request { 
            method: HTTPMethod::NONE,
            resource: String::new(),
            http_version: String::new(),
            content_length: 0,
            content: String::new(),
        };

        let mut i:i32 = 0;

        loop {
            l = String::new();
            reader.by_ref().read_line(&mut l).unwrap();
            l = l.trim_right().to_string();

            println!("{}", l);
            
            if i == 0 {
                let mut iter = l.split_whitespace();
                request.method = match iter.next().unwrap() {
                    "GET" => HTTPMethod::GET,
                    "POST" => HTTPMethod::POST,
                    "PUT" => HTTPMethod::PUT,
                    "HEAD" => HTTPMethod::HEAD,
                    "OPTIONS" => HTTPMethod::OPTIONS,
                    "DELETE" => HTTPMethod::DELETE,
                    m => HTTPMethod::Other(m.to_string()),
                };
                request.resource = iter.next().unwrap().to_string();
                request.http_version = iter.next().unwrap().to_string();
            }
            match l.as_str() {
                "" => {
                    println!("breaking now {}", i);
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

            i += 1;
        }

        if request.content_length > 0 {
            let mut buf = String::new();
            let mut handle = reader.by_ref().take(request.content_length);
            println!("{}", request.content_length);
            let read = handle.read_to_string(&mut buf).unwrap();
            println!("Content: {:?}, {}", buf, read);
            request.content = buf;
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
