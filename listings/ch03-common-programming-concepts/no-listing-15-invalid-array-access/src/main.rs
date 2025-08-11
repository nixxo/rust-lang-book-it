use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Digita un indice dell'array.");

    let mut indice = String::new();

    io::stdin()
        .read_line(&mut indice)
        .expect("Errore di lettura");

    let indice: usize = indice
        .trim()
        .parse()
        .expect("L'indice inserito non è un numero");

    let elemento = a[indice];

    println!("Il valore dell'elemento all'indice {indice} è: {elemento}");
}
