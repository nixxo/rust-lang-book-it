extern crate trpl; // necessario per test mdbook

// ANCHOR: all
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messaggi = ricevi_messaggi();

        while let Some(messaggio) = messaggi.next().await {
            println!("{messaggio}");
        }
    });
}

fn ricevi_messaggi() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messaggi = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for messaggio in messaggi {
        tx.send(format!("Messaggio: '{messaggio}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
// ANCHOR_END: all
