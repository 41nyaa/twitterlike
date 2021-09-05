extern crate http;
extern crate http_bytes;

pub mod db;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result};
use std::io::prelude::*;
use std::fs::File;
use http::{Response, StatusCode};
use http_bytes::response_header_to_vec;
use db::{Tweet, TweetDB, TweetDBSqlite3};

pub struct HttpHandler {
    db : TweetDBSqlite3,
}

impl HttpHandler{
    pub fn new() -> Self {
        let db = TweetDBSqlite3::new();
        db.create();
        HttpHandler{db : db}
    }

    pub fn exec(&self, buf: &[u8]) -> Vec<u8> {
        let res: Vec<u8>;
        if buf.starts_with(b"GET /index.html HTTP/1.1\r\n") {
            res = self.get_index();
        } else if buf.starts_with(b"GET /tweet HTTP/1.1\r\n") {
            res = self.get_tweet();
        } else if buf.starts_with(b"POST /tweet HTTP/1.1\r\n") {
            res = self.post_tweet(&buf);
        } else {
            res = self.not_found();
        }
        res
    }

    pub fn get_index(&self) -> Vec<u8>{
        let mut file = File::open("index.html").unwrap();

        let mut index = String::new();
        file.read_to_string(&mut index).unwrap();

        let header = Response::builder()
            .status(StatusCode::OK)
            .body(()).unwrap();
        let mut res = response_header_to_vec(&header);
        let mut contents = index.into_bytes();
        res.append(&mut contents);
        return res
    }

    pub fn post_tweet(&self, buffer : &[u8]) -> Vec<u8> {
        println!("{}", std::str::from_utf8(&buffer).unwrap());
        let mut headers_buffer = vec![http_bytes::EMPTY_HEADER; 20];
        let (r, b) = http_bytes::parse_request_header(
            buffer,
            &mut headers_buffer[..],
            Some(http_bytes::http::uri::Scheme::HTTP),
        ).unwrap().unwrap();

        let body = std::str::from_utf8(&b[1..b.len()-1]).unwrap();
        println!("{}", body);
        let parsed : Tweet = serde_json::from_str(body).unwrap();

        self.db.post(parsed);
        self.get_tweet()
    }

    pub fn get_tweet(&self) -> Vec<u8> {
        let records = self.db.get();
        let mut body : String = "[".to_string();
        let records_max = records.len();
        for (i, record) in records.iter().enumerate() {
            body += &serde_json::to_string(&record).unwrap();
            if i != (records_max - 1) {
                body += ",";
            }
        }
        body += "]";


        let header = Response::builder()
            .status(StatusCode::OK)
            .header("content-length", body.len())
            .body(()).unwrap();
        let mut res = response_header_to_vec(&header);
        res.append(&mut body.into_bytes());
        res
    }

    pub fn not_found(&self) -> Vec<u8> {
        let res = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(()).unwrap();
        response_header_to_vec(&res)    
    }
}

fn handle_client(mut stream: TcpStream, handler: &HttpHandler) {
    let mut buf= [0;1024];
    let byte = stream.read(&mut buf).unwrap();

    let res = String::from_utf8(handler.exec(&buf[..byte])).unwrap();
    println!("{}", res);
    stream.write(&res.into_bytes()).unwrap();
    stream.flush().unwrap();    
}

pub fn start_server() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let handler   = HttpHandler::new();

    for stream in listener.incoming() {
        handle_client(stream?, &handler);
    }
    Result::Ok(())
}
