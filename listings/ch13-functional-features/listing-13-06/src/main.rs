use std::thread;

fn main() {
    let lista = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {lista:?}");

    thread::spawn(move || println!("Dal thread: {lista:?}"))
        .join()
        .unwrap();
}
