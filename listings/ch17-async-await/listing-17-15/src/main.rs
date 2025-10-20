extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: slow-futures
        let a = async {
            println!("'a' iniziata.");
            lenta("a", 30);
            lenta("a", 10);
            lenta("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finita.");
        };

        let b = async {
            println!("'b' iniziata.");
            lenta("b", 75);
            lenta("b", 10);
            lenta("b", 15);
            lenta("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finita.");
        };

        trpl::select(a, b).await;
        // ANCHOR_END: slow-futures
    });
}

fn lenta(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguita per {ms}ms");
}
