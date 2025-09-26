extern crate trpl; // necessario per test mdbook

use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // ANCHOR: main
        let messaggi = ricevi_messaggi().timeout(Duration::from_millis(200));
        let intervalli = ricevi_intervalli()
            .map(|conteggio| format!("Intervallo: {conteggio}"))
            .timeout(Duration::from_secs(10));
        let uniti = messaggi.merge(intervalli);
        let mut stream = pin!(uniti);

        while let Some(risultato) = stream.next().await {
            // ANCHOR_END: main
            match risultato {
                Ok(messaggio) => println!("{messaggio}"),
                Err(ragione) => eprintln!("Problema: {ragione:?}"),
            }
        }
    })
}

fn ricevi_messaggi() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messaggi = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (indice, messaggio) in messaggi.into_iter().enumerate() {
            let tempo_dormita = if indice % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(tempo_dormita)).await;

            tx.send(format!("Messaggio: '{messaggio}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

fn ricevi_intervalli() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut conteggio = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            conteggio += 1;
            tx.send(conteggio).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
