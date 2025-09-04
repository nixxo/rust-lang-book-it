fn main() {
    let list = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {list:?}");

    let solo_prestito = || println!("Dalla chiusura: {list:?}");

    println!("Prima di chiamare la chiusura: {list:?}");
    solo_prestito();
    println!("Dopo aver chiamato la chiusura: {list:?}");
}
