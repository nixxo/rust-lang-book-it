// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn leggi_nomeutente_dal_file() -> Result<String, io::Error> {
    let nomeutente_file_result = File::open("hello.txt");

    let mut nomeutente_file = match nomeutente_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut nomeutente = String::new();

    match nomeutente_file.read_to_string(&mut nomeutente) {
        Ok(_) => Ok(nomeutente),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let nomeutente = leggi_nomeutente_dal_file().expect("Impossibile ottenere il nome utente");
}
