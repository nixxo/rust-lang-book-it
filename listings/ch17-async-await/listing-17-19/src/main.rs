extern crate trpl; // necessario per test mdbook

// ANCHOR: here
use std::pin::{Pin, pin};

// --taglio--

// ANCHOR_END: here
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // ANCHOR: here
        let tx1_fut = pin!(async move {
            // --taglio--
            // ANCHOR_END: here
            let valori = vec![
                String::from("ciao"),
                String::from("dalla"),
                String::from("future"),
                String::from("!!!"),
            ];

            for val in valori {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        // ANCHOR_END: here
        // ANCHOR: here
        let rx_fut = pin!(async {
            // --taglio--
            // ANCHOR_END: here
            while let Some(valore) = rx.recv().await {
                println!("ricevuto '{valore}'");
            }
            // ANCHOR: here
        });

        let tx_fut = pin!(async move {
            // --taglio--
            // ANCHOR_END: here
            let valori = vec![
                String::from("altri"),
                String::from("messaggi"),
                String::from("per"),
                String::from("te"),
            ];

            for val in valori {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        let future: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        // ANCHOR_END: here

        trpl::join_all(future).await;
    });
}
