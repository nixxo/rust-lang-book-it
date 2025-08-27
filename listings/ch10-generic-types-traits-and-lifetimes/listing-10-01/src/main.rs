// ANCHOR: here
fn main() {
    let lista_numeri = vec![34, 50, 25, 100, 65];

    let mut maggiore = &lista_numeri[0];

    for numero in &lista_numeri {
        if numero > maggiore {
            maggiore = numero;
        }
    }

    println!("Il numero maggiore è {maggiore}");
    // ANCHOR_END: here
    assert_eq!(*maggiore, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
