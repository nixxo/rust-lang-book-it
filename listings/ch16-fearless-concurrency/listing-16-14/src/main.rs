use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let contatore = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contatore = Rc::clone(&contatore);
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
