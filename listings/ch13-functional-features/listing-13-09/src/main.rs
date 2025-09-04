#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let mut list = [
        Rettangolo { larghezza: 10, altezza: 1 },
        Rettangolo { larghezza: 3, altezza: 5 },
        Rettangolo { larghezza: 7, altezza: 12 },
    ];  

    let mut numero_operazioni_sort = 0;
    list.sort_by_key(|r| {
        numero_operazioni_sort += 1;
        r.larghezza
    });
    println!("{list:#?}, ordinato in {numero_operazioni_sort} operazioni");
}
