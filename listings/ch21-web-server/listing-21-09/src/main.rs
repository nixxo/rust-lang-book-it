use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        gestisci_connessione(stream);
    }
}
// ANCHOR: here
// --taglio--

fn gestisci_connessione(mut stream: TcpStream) {
    // --taglio--
    // ANCHOR_END: here
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // ANCHOR: here

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "ciao.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contenuto = fs::read_to_string(filename).unwrap();
    let lunghezza = contenuto.len();

    let risposta =
        format!("{status_line}\r\nContent-Length: {lunghezza}\r\n\r\n{contenuto}");

    stream.write_all(risposta.as_bytes()).unwrap();
}
// ANCHOR_END: here
