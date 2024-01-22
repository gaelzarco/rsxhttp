use std::net::TcpListener;

fn main() {
    println!("Simple HTTP server built for Rust from scratch.");
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "5001";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    // Listner for incoming TCP streams
    let listener = TcpListener::bind(endpoint.clone()).expect("Endpoint is not valid");

    for stream in listener.incoming() {
        let stream = stream.expect("Incoming stream dropped");

        println!("{endpoint} pinged!");
    }
}
