fn main() {
    // ANCHOR: here
    let numeri = (2, 4, 8, 16, 32);

    match numeri {
        (primo, _, terzo, _, quinto) => {
            println!("Alcuni numeri: {primo}, {terzo}, {quinto}");
        }
    }
    // ANCHOR_END: here
}
