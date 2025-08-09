use std::io;

fn main() {
    println!("Indovina il numero!");

    println!("Inserisci la tua ipotesi.");

    let mut ipotesi = String::new();

    io::stdin().read_line(&mut ipotesi);

    println!("Hai ipotizzato: {ipotesi}"); 
}
