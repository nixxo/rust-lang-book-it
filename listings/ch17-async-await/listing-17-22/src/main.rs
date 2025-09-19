extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // Pià tardi chiameremo `lento` da qui
    });
}

// ANCHOR: slow
fn lento(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguito per {ms}ms");
}
// ANCHOR_END: slow
