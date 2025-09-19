extern crate trpl; // necessario per test mdbook

use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messaggi =
            pin!(ricevi_messaggi().timeout(Duration::from_millis(200)));

        while let Some(risultato) = messaggi.next().await {
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

// ANCHOR: intervals
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
// ANCHOR_END: intervals
