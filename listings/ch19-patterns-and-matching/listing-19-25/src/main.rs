fn main() {
    let numeri = (2, 4, 8, 16, 32);

    match numeri {
        (.., secondo, ..) => {
            println!("Alcuni numeri: {secondo}")
        },
    }
}
