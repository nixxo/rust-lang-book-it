use guessing_game::Guess;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod guessing_game;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Per favore, inserisci la tua ipotesi.");

        let mut ipotesi = String::new();

        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Errore a leggere la prima riga");

        let ipotesi: i32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ipotesi = Ipotesi::new(ipotesi);

        match ipotesi.valore().cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal => {
                println!("Hai vinto!");
                break;
            }
        }
    }
}
