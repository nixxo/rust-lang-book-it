use crate::giardino::verdure::Asparagi;

pub mod giardino;

fn main() {
    let pianta = Asparagi {};
    println!("Sto coltivando {pianta:?}!");
}
