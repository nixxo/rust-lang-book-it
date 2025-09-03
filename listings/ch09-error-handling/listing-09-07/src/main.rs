// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn leggi_nomeutente_dal_file() -> Result<String, io::Error> {
    let mut nomeutente_file = File::open("ciao.txt")?;
    let mut nomeutente = String::new();
    nomeutente_file.read_to_string(&mut nomeutente)?;
    Ok(nomeutente)
}
// ANCHOR_END: here

fn main() {
    let nomeutente = leggi_nomeutente_dal_file().expect("Impossibile ottenere il nome utente");
}
