extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let un_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' iniziato.");
            lento("a", 30);
            trpl::sleep(un_ms).await;
            lento("a", 10);
            trpl::sleep(un_ms).await;
            lento("a", 20);
            trpl::sleep(un_ms).await;
            println!("'a' finito.");
        };

        let b = async {
            println!("'b' iniziato.");
            lento("b", 75);
            trpl::sleep(un_ms).await;
            lento("b", 10);
            trpl::sleep(un_ms).await;
            lento("b", 15);
            trpl::sleep(un_ms).await;
            lento("b", 350);
            trpl::sleep(un_ms).await;
            println!("'b' finito.");
        };
        // ANCHOR_END: here

        trpl::race(a, b).await;
    });
}

fn lento(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguito per {ms}ms");
}
