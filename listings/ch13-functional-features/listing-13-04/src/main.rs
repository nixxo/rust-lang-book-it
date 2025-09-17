fn main() {
    let lista = vec![1, 2, 3];
    println!("Prima di definire la chiusura: {lista:?}");

    let solo_prestito = || println!("Dalla chiusura: {lista:?}");

    println!("Prima di chiamare la chiusura: {lista:?}");
    solo_prestito();
    println!("Dopo aver chiamato la chiusura: {lista:?}");
}
