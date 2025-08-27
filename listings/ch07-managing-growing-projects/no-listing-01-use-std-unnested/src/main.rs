use rand::Rng;
// ANCHOR: here
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// ANCHOR_END: here

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    println!("Il numero segreto è: {numero_segreto}");

    println!("Inserisci la tua ipotesi.");

    let mut ipotesi = String::new();

    io::stdin()
        .read_line(&mut ipotesi)
        .expect("Errore di lettura");

    println!("Hai ipotizzato: {ipotesi}");

    match ipotesi.cmp(&numero_segreto) {
        Ordering::Less => println!("Troppo piccolo!"),
        Ordering::Greater => println!("Troppo grande!"),
        Ordering::Equal => println!("Hai indovinato!"),
    }
}
