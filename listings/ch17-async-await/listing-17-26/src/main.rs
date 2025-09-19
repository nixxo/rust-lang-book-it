extern crate trpl; // necessario per test mdbook

use std::time::{Duration, Instant};

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let un_ns = Duration::from_nanos(1);
        let inizio = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(un_ns).await;
            }
        }
        .await;
        let tempo = Instant::now() - inizio;
        println!(
            "versione 'sleep' finita dopo {} secondi.",
            tempo.as_secs_f32()
        );

        let inizio = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let tempo = Instant::now() - inizio;
        println!(
            "versione 'yield' finita dopo {} secondi.",
            tempo.as_secs_f32()
        );
        // ANCHOR_END: here
    });
}
