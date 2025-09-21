use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let valori = vec![
            String::from("ciao"),
            String::from("dal"),
            String::from("thread"),
            String::from("!!!"),
        ];

        for valore in valori {
            tx.send(valore).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for ricevuto in rx {
        println!("Ricevuto: {ricevuto}");
    }
}
