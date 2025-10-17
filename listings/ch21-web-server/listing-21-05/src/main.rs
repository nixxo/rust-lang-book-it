// ANCHOR: here
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
// --taglio--

// ANCHOR_END: here
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        gestisci_connessione(stream);
    }
}

// ANCHOR: here
fn gestisci_connessione(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contenuto = fs::read_to_string("ciao.html").unwrap();
    let lunghezza = contenuto.len();

    let risposta =
        format!("{status_line}\r\nContent-Length: {lunghezza}\r\n\r\n{contenuto}");

    stream.write_all(risposta.as_bytes()).unwrap();
}
// ANCHOR_END: here
