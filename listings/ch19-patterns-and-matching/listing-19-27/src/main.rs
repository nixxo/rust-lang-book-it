fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Ricevuto 50"),
        Some(n) if n == y => println!("Corrisponde, n = {n}"),
        _ => println!("Caso predefinito, x = {x:?}"),
    }

    println!("alla fine: x = {x:?}, y = {y}");
}
