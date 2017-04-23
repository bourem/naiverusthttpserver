use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
// TODO: use HashMap for Request/Response headers
// use std::collections::HashMap;

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
        CONNECT,
        TRACE,
        NONE,
        Other(String),
    }
    
    impl Default for HTTPMethod {
        fn default() -> HTTPMethod { HTTPMethod::NONE }
    }

    #[derive(Debug)]
    #[derive(Default)]
    struct Request {
        method: HTTPMethod,
        request_target: String,
        http_version: String,
        content_length: Option<u64>,
        content: Option<String>,
    }

    #[derive(Debug)]
    #[derive(Default)]
    struct Response {
        http_version: String,
        status_code: String,
        reason_phrase: String,
        content_type: String,
        content: String,
    }

    impl Response {
        // couldn't figure how to make &[u8] + as_bytes() work,
        // instead of Vec<u8> + into_bytes()
        fn to_bytes(&self) -> Vec<u8> {
            let content_length = self.content.len();
            let bytes = format!("{} {} {}\r\n\
                                Content-Type: {}\r\n\
                                Content-Length: {}\r\n\r\n{}", 
                self.http_version, 
                self.status_code, 
                self.reason_phrase,
                self.content_type,
                content_length,
                self.content,
                ).into_bytes();
            bytes
        }
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
                    "CONNECT" => HTTPMethod::CONNECT,
                    "TRACE" => HTTPMethod::TRACE,
                    m => HTTPMethod::Other(m.to_string()),
                };
                request.request_target = iter.next().unwrap().to_string();
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
                            request.content_length = match cl.parse() {
                                Ok(l) => Some(l),
                                Err(_) => None,
                            };
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
        let mut request: Request = Default::default();
        
        read_request_headers(&mut reader, &mut request);

        read_request_body(&mut reader, &mut request);
        
        let mut stream = stream;
        stream.flush().expect("stream couldn't be flushed");
        println!("finished reading");
        
        request
    }

    fn build_response(request: &Request) -> Response {
        let content = match request.request_target.as_str() {
            "/" => include_str!("test.html").to_string(),
            resource => {
                match File::open(resource.trim_left_matches("/")) {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        file.read_to_string(&mut contents);
                        contents
                    },
                    Err(why) => {
                        format!("couldn't read {}: {}", 
                                         resource, 
                                         why.description())
                    },
                }
            },
        };
        Response { 
            http_version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            reason_phrase: "OK".to_string(),
            content_type: "text/html".to_string(),
            content: content.to_string(),
        }
    }

    fn handle_client(mut stream: TcpStream) {
        println!("Hello, world!");

        let request = read_request(&stream);
        println!("{:?}", request);

        let response = build_response(&request);
        println!("{:?}", response.to_bytes());
        
        stream.write_all(response.to_bytes().as_slice()).expect("write failed");

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
