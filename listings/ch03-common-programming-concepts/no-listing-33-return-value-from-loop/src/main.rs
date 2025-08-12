fn main() {
    let mut contatore = 0;

    let risultato = loop {
        contatore += 1;

        if contatore == 10 {
            break contatore * 2;
        }
    };

    println!("Il risultato è {risultato}");
}
