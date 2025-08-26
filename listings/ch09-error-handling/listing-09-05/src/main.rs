use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Si è verificato un errore durante la creazione del file: {e:?}"),
            },
            _ => {
                panic!("Si è verificato un errore durante l'apertura del file: {error:?}");
            }
        },
    };
}
