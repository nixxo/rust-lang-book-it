use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    println!("Il numero segreto è: {numero_segreto}");

    println!("Inserisci la tua ipotesi.");

    // ANCHOR: here
    // --snip--

    let mut ipotesi = String::new();

    io::stdin()
        .read_line(&mut ipotesi)
        .expect("Errore di lettura");

    let ipotesi: u32 = ipotesi.trim().parse().expect("Inserisci un numero!");

    println!("Hai ipotizzato: {ipotesi}");

    match ipotesi.cmp(&numero_segreto) {
        Ordering::Less => println!("Troppo piccolo!"),
        Ordering::Greater => println!("Troppo grande!"),
        Ordering::Equal => println!("Hai indovinato!"),
    }
    // ANCHOR_END: here
}
