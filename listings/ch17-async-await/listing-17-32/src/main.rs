extern crate trpl; // necessario per test mdbook

// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let valori = 1..101;
        let iter = valori.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtrato =
            stream.filter(|val| val % 3 == 0 || val % 5 == 0);

        while let Some(valore) = filtrato.next().await {
            println!("Il valore era: {valore}");
        }
    });
}
// ANCHOR_END: all
