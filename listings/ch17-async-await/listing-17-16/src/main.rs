extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let un_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' iniziata.");
            lenta("a", 30);
            trpl::sleep(un_ms).await;
            lenta("a", 10);
            trpl::sleep(un_ms).await;
            lenta("a", 20);
            trpl::sleep(un_ms).await;
            println!("'a' finita.");
        };

        let b = async {
            println!("'b' iniziata.");
            lenta("b", 75);
            trpl::sleep(un_ms).await;
            lenta("b", 10);
            trpl::sleep(un_ms).await;
            lenta("b", 15);
            trpl::sleep(un_ms).await;
            lenta("b", 350);
            trpl::sleep(un_ms).await;
            println!("'b' finita.");
        };
        // ANCHOR_END: here

        trpl::select(a, b).await;
    });
}

fn lenta(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguita per {ms}ms");
}
