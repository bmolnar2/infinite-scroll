use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let line = buf_reader.lines().next().unwrap().unwrap();

    if line == "GET / HTTP/1.1" {
        stream.write_all(index().as_bytes()).unwrap()
    } else {
        println!("Not found: {line}");
        stream.write_all(not_found().as_bytes()).unwrap()
    }

}

fn not_found() -> String {
    get_page("HTTP/1.1 404 NOT FOUND", "404.html")
}

fn index() -> String {
    get_page("HTTP/1.1 200 OK", "hello.html")
}

fn get_page( status_line: &str, path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

