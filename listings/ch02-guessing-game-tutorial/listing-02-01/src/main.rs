// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Indovina il numero!");

    println!("Inserisci la tua ipotesi.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut ipotesi = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut ipotesi)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Errore di lettura");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Hai ipotizzato: {ipotesi}");
    // ANCHOR_END: print_guess
}
// ANCHOR: all
