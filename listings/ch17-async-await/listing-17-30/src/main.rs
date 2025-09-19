extern crate trpl; // necessario per test mdbook

fn main() {
    trpl::run(async {
        // ANCHOR: stream
        let valori = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = valori.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(valore) = stream.next().await {
            println!("Il valore era: {valore}");
        }
        // ANCHOR_END: stream
    });
}
