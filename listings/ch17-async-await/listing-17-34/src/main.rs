extern crate trpl; // necessario per test mdbook

// ANCHOR: timeout
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
// ANCHOR_END: timeout

fn ricevi_messaggi() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messaggi = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for messaggio in messaggi {
        tx.send(format!("Messaggio: '{messaggio}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
