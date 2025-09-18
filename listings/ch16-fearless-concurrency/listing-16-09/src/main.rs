use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("ciao");
        tx.send(val).unwrap();
        println!("val è {val}");
    });

    let ricevuto = rx.recv().unwrap();
    println!("Ricevuto: {ricevuto}");
}
