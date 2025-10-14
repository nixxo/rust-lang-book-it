fn main() {
    // ANCHOR: here
    enum Messaggio {
        Ciao { id: i32 },
    }

    let msg = Messaggio::Ciao { id: 5 };

    match msg {
        Messaggio::Ciao { id: id @ 3..=7 } => {
            println!("Trovato un id nell'intervallo: {id}")
        }
        Messaggio::Ciao { id: 10..=12 } => {
            println!("Trovato un id in un altro intervallo")
        }
        Messaggio::Ciao { id } => println!("Trovato un altro id: {id}"),
    }
    // ANCHOR_END: here
}
