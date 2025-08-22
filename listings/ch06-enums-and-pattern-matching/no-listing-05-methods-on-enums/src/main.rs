fn main() {
    enum Messaggio {
        Esci,
        Sposta { x: i32, y: i32 },
        Scrivi(String),
        CambiaColore(i32, i32, i32),
    }

    // ANCHOR: here
    impl Messaggio {
        fn chiama(&self) {
            // method body would be defined here
        }
    }

    let m = Messaggio::Scrivi(String::from("ciao"));
    m.chiama();
    // ANCHOR_END: here
}
