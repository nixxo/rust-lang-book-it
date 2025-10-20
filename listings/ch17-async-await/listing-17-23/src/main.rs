extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
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
        };

        let rx_fut = async {
            while let Some(valore) = rx.recv().await {
                println!("ricevuto '{valore}'");
            }
        };

        // ANCHOR: here
        let tx_fut = async move {
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
        };

        let future: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(future).await;
        // ANCHOR_END: here
    });
}
