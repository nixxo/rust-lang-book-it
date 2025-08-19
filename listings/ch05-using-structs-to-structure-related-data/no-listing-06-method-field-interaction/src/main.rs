#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

// ANCHOR: here
impl Rettangolo {
    fn larghezza(&self) -> bool {
        self.larghezza > 0
    }
}

fn main() {
    let rettangolo1 = Rettangolo {
        larghezza: 30,
        altezza: 50,
    };

    if rettangolo1.larghezza() {
        println!("La larghezza del rettangolo è > 0; è {}", rettangolo1.larghezza);
    }
}
// ANCHOR_END: here
