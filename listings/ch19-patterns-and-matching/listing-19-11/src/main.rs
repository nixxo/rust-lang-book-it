fn main() {
    // ANCHOR: here
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Ricevuto 50"),
        Some(y) => println!("Corrisponde, y = {y}"),
        _ => println!("Caso predefinito, x = {x:?}"),
    }

    println!("alla fine: x = {x:?}, y = {y}");
    // ANCHOR_END: here
}
