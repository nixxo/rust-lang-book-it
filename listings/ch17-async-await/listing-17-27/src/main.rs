extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let lento = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finalmente finito"
        };

        match timeout(lento, Duration::from_secs(2)).await {
            Ok(messaggio) => println!("Completato con '{messaggio}'"),
            Err(durata) => {
                println!("Fallito dopo {} secondi", durata.as_secs())
            }
        }
        // ANCHOR_END: here
    });
}
