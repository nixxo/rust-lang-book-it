use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    println!("Il numero segreto è: {numero_segreto}");

    loop {
        println!("Inserisci la tua ipotesi.");

        let mut ipotesi = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Errore di lettura");

        // ANCHOR: ch19
        let ipotesi: u32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Hai ipotizzato: {ipotesi}");

        // --snip--
        // ANCHOR_END: here

        match ipotesi.cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            }
        }
    }
}
