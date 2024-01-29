use std::{
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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
