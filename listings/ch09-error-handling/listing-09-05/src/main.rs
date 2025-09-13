use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_benvenuto_result = File::open("ciao.txt");

    let file_benvenuto = match file_benvenuto_result {
        Ok(file) => file,
        Err(errore) => match errore.kind() {
            ErrorKind::NotFound => match File::create("ciao.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Errore nella creazione del file: {e:?}"),
            },
            _ => {
                panic!("Errore nell'apertura del file: {errore:?}");
            }
        },
    };
}
