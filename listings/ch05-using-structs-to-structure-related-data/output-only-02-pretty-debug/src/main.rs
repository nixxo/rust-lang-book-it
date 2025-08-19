#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let rettangolo1 = Rettangolo {
        larghezza: 30,
        altezza: 50,
    };

    println!("rettangolo1 è {rettangolo1:#?}");
}
