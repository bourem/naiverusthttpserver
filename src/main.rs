use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;

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
        content_length: Option<u64>,
        content: Option<String>,
    }

    fn read_request_headers<R: Read>(reader: &mut BufReader<R>, request: &mut Request) {
        let mut l: String;
        
        let mut i:i32 = 0;

        loop {
            l = String::new();
            reader.by_ref().read_line(&mut l).unwrap();
            l = l.trim_right().to_string();

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
                    break;
                },
                a => {
                    let mut iter = a.split_whitespace();
                    match iter.next().unwrap() {
                        "Content-Length:" => {
                            let cl = iter.next().unwrap();
                            request.content_length = Some(cl.parse().unwrap());
                        },
                        _ => {
                        },
                    }
                },
            }

            i += 1;
        }
    }

    fn read_request_body<R: Read>(reader: &mut BufReader<R>, request: &mut Request) {
        match request.content_length {
            Some(l) => {
                let mut buf = String::new();
                let mut handle = reader.by_ref().take(l);
                let read = handle.read_to_string(&mut buf).unwrap();
                assert_eq!(l, read as u64);
                request.content = Some(buf);
            },
            None => {
            },
        }
    }

    fn read_request(stream: &TcpStream) -> Request {
        let mut reader = BufReader::new(stream);

        let mut request = Request { 
            method: HTTPMethod::NONE,
            resource: String::new(),
            http_version: String::new(),
            content_length: None,
            content: None,
        };

        read_request_headers(&mut reader, &mut request);

        read_request_body(&mut reader, &mut request);
        
        let mut stream = stream;
        stream.flush().expect("stream couldn't be flushed");
        println!("finished reading");
        
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
