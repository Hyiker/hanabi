/*
    Hanabi HTTP server by hyiker.
    Powered by Rust language.
    Keep fool, keep hungry.
*/

use hanabi::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
fn main() {
    // returns a TcpListener instance(wrapped by Result<T,E>)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    // iterate to fetch the incoming tcp connection
    // store the tcp connection inside stream
    // it's currently synchronized.
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    // create an buffer array len 1024 to store the char stream
    // beware, that if the size of the buffer is too small for the request
    // such that it's not able to carry the request message,
    // the http connection may fail
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (get, sleep) = (b"GET / HTTP/1.1\r\n", b"GET /sleep HTTP/1.1\r\n");

    let (filename, status_line) = if buffer.starts_with(get) {
        // return properly only when the request matches "get"
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_millis(10000));
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };
    let body = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
