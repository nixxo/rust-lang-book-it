#[derive(Debug)]
struct Rectangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let mut list = [
        Rectangolo { larghezza: 10, altezza: 1 },
        Rectangolo { larghezza: 3, altezza: 5 },
        Rectangolo { larghezza: 7, altezza: 12 },
    ];

    list.sort_by_key(|r| r.larghezza);
    println!("{list:#?}");
}
