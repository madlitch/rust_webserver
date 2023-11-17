mod database;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use rusqlite::Error;

use webserver::ThreadPool;
use serde_json;
use crate::database::*;


const ADDRESS: &str = "localhost";
const PORT: &str = "8888";
const WORKERS: usize = 4;


fn main() {
    init_database().unwrap();
    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}")).unwrap();

    let pool = ThreadPool::new(WORKERS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();

    print!("{}", request_line);

    // determine the request type
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();

    // deconstruct url into parts
    let mut url: Vec<&str> = parts.next().unwrap().split("/").collect();

    // grab last part as query then reconstruct url
    let mut query = "";
    if url.len() > 2 {
        query = url.pop().unwrap();
    }
    let path = &*url.join("/");

    // get content length if request has a body
    let mut content_length: usize = 0;
    loop {
        let mut header_line = String::new();
        buf_reader.read_line(&mut header_line).unwrap();
        if header_line == "\r\n" {
            break;
        }

        if header_line.starts_with("Content-Length:") {
            let parts: Vec<&str> = header_line.split_whitespace().collect();
            content_length = parts[1].parse::<usize>().unwrap_or(0);
        }
    }

    // allocate status_line and response_body by matching method and path to specific endpoints
    let (status_line, response_body) = match (method, path) {
        ("GET", "/") => {
            ("HTTP/1.1 200 OK", "Hello, world!".to_string())
        }
        ("GET", "/notes") => {
            // match return from database with either note, no return, or error
            match get_from_database(query) {
                Ok(note) => {
                    ("HTTP/1.1 200 OK", serde_json::to_string(&note).unwrap())
                }
                Err(Error::QueryReturnedNoRows) => {
                    ("HTTP/1.1 200 OK", "Note not found".to_string())
                }
                Err(_) => {
                    ("HTTP/1.1 501 ERROR", "Server Error".to_string())
                }
            }
        }
        ("POST", "/notes") => {
            let body: serde_json::Value = get_body(&mut buf_reader, content_length);
            insert_into_database(
                body["title"].as_str().unwrap(),
                body["content"].as_str().unwrap(),
            ).unwrap();
            ("HTTP/1.1 200 OK", "Note inserted".to_string())
        }
        ("PUT", "/notes") => {
            let body: serde_json::Value = get_body(&mut buf_reader, content_length);
            update_database(
                query,
                body["title"].as_str().unwrap(),
                body["content"].as_str().unwrap(),
            ).unwrap();
            ("HTTP/1.1 200 OK", "Note updated".to_string())
        }
        ("DELETE", "/notes") => {
            delete_from_database(query).unwrap();
            ("HTTP/1.1 200 OK", "Note deleted".to_string())
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "Not found".to_string()),
    };

    // format response body
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_body(buf_reader: &mut BufReader<&mut TcpStream>, content_length: usize) -> serde_json::Value {
    let mut body = String::new();
    buf_reader.take(content_length as u64).read_to_string(&mut body).unwrap();
    serde_json::from_str(&body).unwrap()
}
