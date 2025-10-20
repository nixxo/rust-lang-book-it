extern crate trpl; // necessario per test mdbook

use std::time::Duration;

// ANCHOR: implementation
use trpl::Either;

// --taglio--

// ANCHOR_END: implementation
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

// ANCHOR: implementation
async fn timeout<F: Future>(
    future_da_testare: F,
    tempo_massimo: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_da_testare, trpl::sleep(tempo_massimo)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(tempo_massimo),
    }
}
// ANCHOR_END: implementation
