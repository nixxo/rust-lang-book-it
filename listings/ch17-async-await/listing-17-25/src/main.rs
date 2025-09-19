extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: yields
        let a = async {
            println!("'a' iniziato.");
            lento("a", 30);
            trpl::yield_now().await;
            lento("a", 10);
            trpl::yield_now().await;
            lento("a", 20);
            trpl::yield_now().await;
            println!("'a' finito.");
        };

        let b = async {
            println!("'b' iniziato.");
            lento("b", 75);
            trpl::yield_now().await;
            lento("b", 10);
            trpl::yield_now().await;
            lento("b", 15);
            trpl::yield_now().await;
            lento("b", 350);
            trpl::yield_now().await;
            println!("'b' finito.");
        };
        // ANCHOR_END: yields

        trpl::race(a, b).await;
    });
}

fn lento(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguito per {ms}ms");
}
