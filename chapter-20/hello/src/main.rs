use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // `TcpListener` will listen for TCP connections at the passed-in address string
    // bind() works like the new() function, it will return a new `TcpListener` instance
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming() returns an iterator that gives us a sequence of streams of type `TcpStream` (a stream represents a connection between a client and server)
    for stream in listener.incoming() {
        // Read from the `TcpStream` to see what the client sent
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connections(stream);
        });
    }
}

fn handle_connections(mut stream: TcpStream) {
    // `BufReader` adds buffering by managing calls to the `std::io::Read` trait
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // Send the response to the client
    stream.write_all(response.as_bytes()).unwrap();
}
