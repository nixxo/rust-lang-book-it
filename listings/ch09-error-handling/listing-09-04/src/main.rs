use std::fs::File;

fn main() {
    let file_benvenuto_result = File::open("ciao.txt");

    let file_benvenuto = match file_benvenuto_result {
        Ok(file) => file,
        Err(errore) => panic!("Errore nell'apertura del file: {errore:?}"),
    };
}
