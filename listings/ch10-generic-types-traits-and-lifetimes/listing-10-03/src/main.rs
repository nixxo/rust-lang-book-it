// ANCHOR: here
fn maggiore(lista: &[i32]) -> &i32 {
    let mut maggiore = &list[0];

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
    // ANCHOR_END: here
    assert_eq!(*result, 100);
    // ANCHOR: here

    let lista_numeri = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = maggiore(&lista_numeri);
    println!("Il numero maggiore è {result}");
    // ANCHOR_END: here
    assert_eq!(*result, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
