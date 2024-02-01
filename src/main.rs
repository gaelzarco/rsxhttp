use rsxhttp::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "5000";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    // Listner for incoming TCP streams
    // bind function returns an Error<T, E>
    // It can fail because some ports require admin access
    let listener = TcpListener::bind(endpoint.clone()).expect("Endpoint is not valid");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.expect("Stream could not be parsed");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("TcpStream iterator is empty")
        .expect("Could not parse string");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 OK 200", "src/hello.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "src/err.html"),
    };
    let contents = fs::read_to_string(filename).expect("Failed to open specified file");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .expect("Could not write stream");
}
