fn main() {
    let numeri = (2, 4, 8, 16, 32);

    match numeri {
        (primo, .., ultimo) => {
            println!("Alcuni numeri: {primo}, {ultimo}");
        }
    }
}
