#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let mut lista = [
        Rettangolo { larghezza: 10, altezza: 1 },
        Rettangolo { larghezza: 3, altezza: 5 },
        Rettangolo { larghezza: 7, altezza: 12 },
    ];

    lista.sort_by_key(|r| r.larghezza);
    println!("{lista:#?}");
}
