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

    let mut operazioni_sort = vec![];
    let value = String::from("chiusura chiamata");

    list.sort_by_key(|r| {
        operazioni_sort.push(value);
        r.larghezza
    });
    println!("{list:#?}");
}
