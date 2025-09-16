use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let percorso_file = &args[2];

    println!("Cerco {query}");
    println!("Nel file {percorso_file}");
}
