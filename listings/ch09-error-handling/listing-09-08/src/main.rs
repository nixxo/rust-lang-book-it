// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn leggi_nomeutente_dal_file() -> Result<String, io::Error> {
    let mut nomeutente = String::new();

    File::open("ciao.txt")?.read_to_string(&mut nomeutente)?;

    Ok(nomeutente)
}
// ANCHOR_END: here

fn main() {
    let nomeutente = leggi_nomeutente_dal_file().expect("Impossibile ottenere il nome utente");
}
