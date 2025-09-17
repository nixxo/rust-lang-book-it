fn main() {
    let mut lista = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {lista:?}");

    let mut prestito_mutabile = || lista.push(7);

    prestito_mutabile();
    println!("Dopo aver chiamato la chiusura: {lista:?}");
}
