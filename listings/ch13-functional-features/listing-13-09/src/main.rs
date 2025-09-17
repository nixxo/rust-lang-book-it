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

    let mut numero_azioni_ordinamento = 0;
    lista.sort_by_key(|r| {
        numero_azioni_ordinamento += 1;
        r.larghezza
    });
    println!("{lista:#?}, ordinato in {numero_azioni_ordinamento} azioni");
}
