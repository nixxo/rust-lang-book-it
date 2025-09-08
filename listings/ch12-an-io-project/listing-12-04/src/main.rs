// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --taglio--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Cerco {query}");
    // ANCHOR: here
    println!("Nel file {file_path}");

    let contenuto = fs::read_to_string(file_path)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
}
// ANCHOR_END: here
