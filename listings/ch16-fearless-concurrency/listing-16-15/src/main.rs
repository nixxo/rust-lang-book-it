use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contatore = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contatore = Arc::clone(&contatore);
        let handle = thread::spawn(move || {
            let mut num = contatore.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Risultato: {}", *contatore.lock().unwrap());
}
