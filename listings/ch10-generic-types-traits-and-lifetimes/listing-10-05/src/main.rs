fn maggiore<T>(lista: &[T]) -> &T {
    let mut maggiore = &lista[0];

    for elemento in lista {
        if elemento > maggiore {
            maggiore = elemento;
        }
    }

    maggiore
}

fn main() {
    let lista_numeri = vec![34, 50, 25, 100, 65];

    let result = maggiore(&lista_numeri);
    println!("Il numero maggiore è {result}");

    let lista_caratteri = vec!['y', 'm', 'a', 'q'];

    let result = maggiore(&lista_caratteri);
    println!("Il carattere maggiore è {result}");
}
