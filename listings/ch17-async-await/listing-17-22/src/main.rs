extern crate trpl; // necessario per test mdbook

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // Pià tardi chiameremo `lenta` da qui
    });
}

// ANCHOR: slow
fn lenta(nome: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{nome}' eseguita per {ms}ms");
}
// ANCHOR_END: slow
