struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let rettangolo1 = Rettangolo {
        larghezza: 30,
        altezza: 50,
    };

    println!(
        "L'area del rettangolo è di {} pixel quadrati.",
        area(&rettangolo1)
    );
}

fn area(rettangolo: &Rettangolo) -> u32 {
    rettangolo.larghezza * rettangolo.altezza
}
