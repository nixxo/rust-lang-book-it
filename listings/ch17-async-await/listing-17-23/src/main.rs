extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: slow-futures
        let a = async {
            println!("'a' iniziato.");
            lento("a", 30);
            lento("a", 10);
            lento("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finito.");
        };

        let b = async {
            println!("'b' iniziato.");
            lento("b", 75);
            lento("b", 10);
            lento("b", 15);
            lento("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finito.");
        };

        trpl::race(a, b).await;
        // ANCHOR_END: slow-futures
    });
}

fn lento(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguito per {ms}ms");
}
