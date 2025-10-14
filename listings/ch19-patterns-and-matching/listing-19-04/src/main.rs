fn main() {
    // ANCHOR: here
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(valore) = rx.recv() {
        println!("{valore}");
    }
    // ANCHOR_END: here
}
