// ANCHOR: here
fn maggior_i32(lista: &[i32]) -> &i32 {
    let mut maggiore = &lista[0];

    for elemento in lista {
        if elemento > maggiore {
            maggiore = elemento;
        }
    }

    maggiore
}

fn maggior_char(lista: &[char]) -> &char {
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

    let risultato = maggior_i32(&lista_numeri);
    println!("Il numero maggiore è  {risultato}");
    // ANCHOR_END: here
    assert_eq!(*risultato, 100);
    // ANCHOR: here

    let lista_caratteri = vec!['y', 'm', 'a', 'q'];

    let risultato = maggior_char(&lista_caratteri);
    println!("Il carattere maggiore è  {risultato}");
    // ANCHOR_END: here
    assert_eq!(*risultato, 'y');
    // ANCHOR: here
}
// ANCHOR_END: here
    