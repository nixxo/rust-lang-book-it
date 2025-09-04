fn main() {
    let mut list = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {list:?}");

    let mut prestito_mutabile = || list.push(7);

    prestito_mutabile();
    println!("Dopo aver chiamato la chiusura: {list:?}");
}
