use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Inserisci la tua ipotesi.");

        let mut ipotesi = String::new();

        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Errore di lettura");

        let ipotesi: u32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Hai ipotizzato: {ipotesi}");

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
