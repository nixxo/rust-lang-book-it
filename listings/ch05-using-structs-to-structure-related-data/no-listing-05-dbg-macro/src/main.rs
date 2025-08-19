#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

fn main() {
    let scala = 2;
    let rettangolo1 = Rettangolo {
        larghezza: dbg!(30 * scala),
        altezza: 50,
    };

    dbg!(&rettangolo1 );
}
