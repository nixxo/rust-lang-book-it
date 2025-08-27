fn main() {
    let lista_numeri = vec![34, 50, 25, 100, 65];

    let mut maggiore = &lista_numeri[0];

    for numero in &lista_numeri {
        if numero > maggiore {
            maggiore = numero;
        }
    }

    println!("Il numero maggiore è {maggiore}");

    let lista_numeri = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut maggiore = &lista_numeri[0];

    for numero in &lista_numeri {
        if numero > maggiore {
            maggiore = numero;
        }
    }

    println!("Il numero maggiore è {maggiore}");
}
