use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "5001";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    // Listner for incoming TCP streams
    // bind function returns an Error<T, E>
    // It can fail because some ports require admin access
    let listener = TcpListener::bind(endpoint.clone()).expect("Endpoint is not valid");

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

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 OK";
        let contents = fs::read_to_string("src/hello.html").expect("Failed to open specified file");
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream
            .write_all(response.as_bytes())
            .expect("Could not write stream");
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("src/err.html").expect("Failed to open specified file");
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream
            .write_all(response.as_bytes())
            .expect("Could not write stream");
    }
}
