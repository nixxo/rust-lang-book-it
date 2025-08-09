// ANCHOR: all
use std::io;

// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Indovina il numero!");

    // ANCHOR: ch07-04
    let numero_segreto = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Il numero segreto è: {numero_segreto}");

    println!("Inserisci la tua ipotesi.");

    let mut ipotesi = String::new();

    io::stdin()
        .read_line(&mut ipotesi)
        .expect("Errore di lettura");

    println!("Hai ipotizzato: {ipotesi}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
