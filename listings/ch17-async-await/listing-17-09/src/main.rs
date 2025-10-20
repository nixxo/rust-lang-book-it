extern crate trpl; // necessario per test mdbook

fn main() {
    trpl::block_on(async {
        // ANCHOR: channel
        let (tx, mut rx) = trpl::channel();

        let val = String::from("ciao");
        tx.send(val).unwrap();

        let ricevuto = rx.recv().await.unwrap();
        println!("ricevuto '{ricevuto}'");
        // ANCHOR_END: channel
    });
}
