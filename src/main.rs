// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
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
    let request_line = buffer.lines().next().unwrap().unwrap();

    let (status_line, body) = if request_line.starts_with("GET /echo/") {
        let echo_content = &request_line[10..request_line.len()-9];
        // println!("{:#?}", echo_content);
        ("HTTP/1.1 200 OK", echo_content)
    } else {
        match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", ""),
            _ => ("HTTP/1.1 404 Not Found", ""),
        }
    };

   
    let response = if body.is_empty() {
        format!("{status_line}\r\n\r\n")
    } else {
        let content_length = body.len();
        let content_type = "text/plain"; // Assuming plain text; adjust as necessary
        format!("{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n{body}")
    };

    stream.write_all(response.as_bytes()).expect("200 OK\n");
}
