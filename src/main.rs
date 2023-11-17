use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

const ADDRESS: &str = "localhost";
const PORT: &str = "8888";


fn main() {
    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}")).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {request_line}");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };


    let contents = fs::read_to_string(format!("templates/{filename}")).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn read_json_file(file_path: &str) -> serde_json::Value {
    let data = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&data).unwrap()
}

fn write_json_file(file_path: &str, data: &serde_json::Value) {
    let json_string = serde_json::to_string(data).unwrap();
    fs::write(file_path, json_string).unwrap();
}
