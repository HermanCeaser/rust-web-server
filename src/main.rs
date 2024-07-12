// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let request_line = buffer
        .lines()
        .next()
        .unwrap()
        .unwrap();

    if request_line == "GET / HTTP/1.1" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).expect("Failed to write to Response Object\n");
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).expect("200 OK\n");
    }
}
