extern crate trpl; // necessario per test mdbook

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: handle
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("ciao numero {i} dal primo task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("ciao numero {i} dal secondo task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
        // ANCHOR_END: handle
    });
}
