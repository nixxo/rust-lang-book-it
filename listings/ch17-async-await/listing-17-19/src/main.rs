extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::block_on(async {
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
    });
}

// ANCHOR: declaration
async fn timeout<F: Future>(
    future_da_testare: F,
    tempo_massimo: Duration,
) -> Result<F::Output, Duration> {
    // Qui è dove metteremo l'implementazione!
}
// ANCHOR_END: declaration
