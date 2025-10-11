fn main() {
    // ANCHOR: here
    enum Messaggio {
        Hello { id: i32 },
    }

    let msg = Messaggio::Hello { id: 5 };

    match msg {
        Messaggio::Hello { id: id @ 3..=7 } => {
            println!("Trovato un id nell'intervallo: {id}")
        }
        Messaggio::Hello { id: 10..=12 } => {
            println!("Trovato un id in un altro intervallo")
        }
        Messaggio::Hello { id } => println!("Trovato un altro id: {id}"),
    }
    // ANCHOR_END: here
}
