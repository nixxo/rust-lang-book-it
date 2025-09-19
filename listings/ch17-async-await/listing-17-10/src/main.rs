extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: many-messages
        let (tx, mut rx) = trpl::channel();

        let valori = vec![
            String::from("ciao"),
            String::from("dal"),
            String::from("futuro"),
            String::from("!!!"),
        ];

        for valore in valori {
            tx.send(valore).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(val) = rx.recv().await {
            println!("ricevuto '{val}'");
        }
        // ANCHOR_END: many-messages
    });
}
