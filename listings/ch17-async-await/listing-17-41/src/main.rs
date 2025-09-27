extern crate trpl; // necessario per test mdbook

use std::{pin::pin, thread, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messaggi = ricevi_messaggi().timeout(Duration::from_millis(200));
        let intervalli = ricevi_intervalli()
            .map(|conteggio| format!("Intervallo: {conteggio}"))
            .throttle(Duration::from_millis(500))
            .timeout(Duration::from_secs(10));
        let uniti = messaggi.merge(intervalli).take(20);
        let mut stream = pin!(uniti);

        while let Some(risultato) = stream.next().await {
            match risultato {
                Ok(elemento) => println!("{elemento}"),
                Err(ragione) => eprintln!("Problema: {ragione:?}"),
            }
        }
    });
}

fn ricevi_messaggi() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messaggi = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (indice, messaggio) in messaggi.into_iter().enumerate() {
            let tempo_dormita = if indice % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(tempo_dormita)).await;

            if let Err(errore_invio) = tx.send(format!("Messaggio: '{messaggio}'")) {
                eprintln!("Impossibile inviare messaggio '{messaggio}': {errore_invio}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

// ANCHOR: threads
fn ricevi_intervalli() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // This is *not* `trpl::spawn` but `std::thread::spawn`!
    thread::spawn(move || {
        let mut conteggio = 0;
        loop {
            // Likewise, this is *not* `trpl::sleep` but `std::thread::sleep`!
            thread::sleep(Duration::from_millis(1));
            conteggio += 1;

            if let Err(errore_invio) = tx.send(conteggio) {
                eprintln!("Impossibile inviare intervallo {conteggio}: {errore_invio}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
// ANCHOR_END: threads
