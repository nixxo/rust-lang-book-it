extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: futures
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let valori = vec![
                String::from("ciao"),
                String::from("dalla"),
                String::from("future"),
                String::from("!!!"),
            ];

            for valore in valori {
                tx.send(valore).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("ricevuto '{val}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
        // ANCHOR_END: futures
    });
}
