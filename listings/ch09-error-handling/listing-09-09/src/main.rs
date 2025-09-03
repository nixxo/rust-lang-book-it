// ANCHOR: here
use std::fs;
use std::io;

fn leggi_nomeutente_dal_file() -> Result<String, io::Error> {
    fs::read_to_string("ciao.txt")
}
// ANCHOR_END: here

fn main() {
    let nomeutente = leggi_nomeutente_dal_file().expect("Impossibile ottenere il nome utente");
}
