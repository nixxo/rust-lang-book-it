use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {list:?}");

    thread::spawn(move || println!("Dal thread: {list:?}"))
        .join()
        .unwrap();
}
