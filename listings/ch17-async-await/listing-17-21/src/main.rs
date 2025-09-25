extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let lenta = async {
            println!("'lenta' iniziato.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'lenta' finito.");
        };

        let veloce = async {
            println!("'veloce' iniziato.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'veloce' finito.");
        };

        trpl::race(lenta, veloce).await;
        // ANCHOR_END: here
    });
}
