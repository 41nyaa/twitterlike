extern crate http;

use std::io::prelude::*;
use std::net::{TcpStream};
use http::{Request};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tweet {
    pub name     : String,
    pub value    : String,
}

pub fn post_tweet(tweet: Tweet) -> Vec<Tweet> {
    let mut tweets: Vec<Tweet> = Vec::new();
    tweets.push(tweet);
    let body_string = serde_json::to_string(&tweets).unwrap();
    let request = Request::builder()
        .method("POST")
        .uri("https://127.0.0.1:8080/tweet")
        .body(())
        .unwrap();

    let mut send_buffer = http_bytes::request_header_to_vec(&request);
    send_buffer.append(&mut body_string.into_bytes());

    //send request and receive response
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let _ = stream.write(&send_buffer).unwrap();
    let mut read_buffer = [0;1024];
    let _ = stream.read(&mut read_buffer).unwrap();

    let mut headers_buffer = vec![http_bytes::EMPTY_HEADER; 20];
    let (r, b) = http_bytes::parse_response_header(
        &read_buffer,
        &mut headers_buffer[..]
    ).unwrap().unwrap();

    let len = r.headers().get(http_bytes::http::header::CONTENT_LENGTH).unwrap().to_str().unwrap();
    let body = std::str::from_utf8(&b[0..len.parse().unwrap()]).unwrap();
    let parsed: Vec<Tweet> = serde_json::from_str(&body).unwrap();
    parsed
}