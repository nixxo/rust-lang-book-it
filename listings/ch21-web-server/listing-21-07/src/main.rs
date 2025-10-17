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

fn gestisci_connessione(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contenuto = fs::read_to_string("ciao.html").unwrap();
        let lunghezza = contenuto.len();

        let risposta = format!(
            "{status_line}\r\nContent-Length: {lunghezza}\r\n\r\n{contenuto}"
        );

        stream.write_all(risposta.as_bytes()).unwrap();
    // ANCHOR: here
    // --taglio--
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contenuto = fs::read_to_string("404.html").unwrap();
        let lunghezza = contenuto.len();

        let risposta = format!(
            "{status_line}\r\nContent-Length: {lunghezza}\r\n\r\n{contenuto}"
        );

        stream.write_all(risposta.as_bytes()).unwrap();
    }
    // ANCHOR_END: here
}
