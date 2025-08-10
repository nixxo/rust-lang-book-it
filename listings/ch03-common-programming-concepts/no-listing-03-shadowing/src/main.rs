fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Il valore di x nello scope interno è: {x}");
    }

    println!("Il valore di x è: {x}");
}
