use std::thread;
use std::time::Duration;

fn genera_allenamento(intensità: u32, numero_casuale: u32) {
    // ANCHOR: here
    let chiusura_lenta = |num: u32| -> u32 {
        println!("calcolo lentamente...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // ANCHOR_END: here

    if intensità < 25 {
        println!("Oggi, fai {} flessioni!", chiusura_lenta(intensità));
        println!("Poi, fai {} piegamenti!", chiusura_lenta(intensità));
    } else {
        if numero_casuale == 3 {
            println!("Oggi fai una pausa! Ricordati di idratarti!");
        } else {
            println!(
                "Oggi, corri per {} minuti!",
                chiusura_lenta(intensità)
            );
        }
    }
}

fn main() {
    let simulazione_numero_utente = 10;
    let simulazione_numero_casuale = 7;

    genera_allenamento(simulazione_numero_utente, simulazione_numero_casuale);
}
