use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // --taglio--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("ciao"),
            String::from("dal"),
            String::from("thread"),
            String::from("!!!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("ancora"),
            String::from("messaggi"),
            String::from("per"),
            String::from("te"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for ricevuto in rx {
        println!("Ricevuto: {ricevuto}");
    }

    // --taglio--
    // ANCHOR_END: here
}
