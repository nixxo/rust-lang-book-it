extern crate trpl; // necessario per test mdbook

// ANCHOR: all
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("ciao numero {i} dal primo task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("ciao numero {i} dal secondo task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
// ANCHOR_END: all
