// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_connection(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();
    
    if http_request.is_empty() {
        return;
    }

    let request_line = http_request.get(0).unwrap();
    // let host_line = http_request.get(1).unwrap();
    let user_agent = http_request.iter().find(|line| line.starts_with("User-Agent:"));


    let (status_line, body) = if request_line.starts_with("GET /echo/") {
        let echo_content = &request_line[10..request_line.len()-9];
        ("HTTP/1.1 200 OK", echo_content)
    } else {
        match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", ""),
            "GET /user-agent HTTP/1.1" => {
                if let Some(user_agent) = user_agent {
                    let (_, agent) = user_agent.split_at(12);
                    ("HTTP/1.1 200 OK", agent)
                } else {
                    ("HTTP/1.1 400 Bad Request", "User-Agent header not found")
                }
            },
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
