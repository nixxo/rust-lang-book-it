use std::fs::File;

fn main() {
    let file_benvenuto = File::open("ciao.txt")
        .expect("ciao.txt dovrebbe essere presente in questo progetto");
}
