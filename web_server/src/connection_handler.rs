use std::time::Duration;
use std::net::TcpStream;
use std::{fs, thread};
use std::io::Read;
use std::io::Write;

enum ResponseStatus {
    Ok,
    NotFound,
}

impl ResponseStatus {
    fn get_status_line(&self) -> String {
        let (status_code, status_message) = match self {
            ResponseStatus::Ok => (200, "OK"),
            ResponseStatus::NotFound => (404, "NOT FOUND"),
        };
        format!("HTTP/1.1 {} {}\r\n", status_code, status_message)
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response;
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        response = response_from_file(ResponseStatus::Ok, "hello.html".to_owned());
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        response = response_from_file(ResponseStatus::Ok, "hello.html".to_owned());
    } else {
        response = response_from_file(ResponseStatus::NotFound, "404.html".to_owned());
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn response_from_file(response_status: ResponseStatus, file_name: String) -> String {
    let contents = fs::read_to_string(file_name).unwrap();
    format!(
        "{}Content-Length: {}\r\n\r\n{}",
        response_status.get_status_line(),
        contents.len(),
        contents
    )
}

