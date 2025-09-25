extern crate trpl; // necessario per test mdbook

// ANCHOR: here
use std::pin::Pin;

// --taglio--

// ANCHOR_END: here
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let valori = vec![
                String::from("ciao"),
                String::from("dal"),
                String::from("futuro"),
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

        let tx_fut = async move {
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
        };

        // ANCHOR: here
        let future: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        // ANCHOR_END: here

        trpl::join_all(future).await;
    });
}
