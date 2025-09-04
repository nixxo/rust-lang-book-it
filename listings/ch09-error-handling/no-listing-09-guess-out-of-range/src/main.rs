use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    loop {
        // --taglio--

        // ANCHOR_END: here
        println!("Inserisci la tua ipotesi.");

        let mut ipotesi = String::new();

        io::stdin()
            .read_line(&mut ipotesi)
            .expect("Errore di lettura");

        // ANCHOR: here
        let ipotesi: i32 = match ipotesi.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if ipotesi < 1 || ipotesi > 100 {
            println!("Il numero segreto è compreso tra 1 e 100.");
            continue;
        }

        match ipotesi.cmp(&numero_segreto) {
            // --taglio--
            // ANCHOR_END: here
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
