#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

// ANCHOR: here
impl Rettangolo {
    fn area(&self) -> u32 {
        self.larghezza * self.altezza
    }
}

impl Rettangolo {
    fn puo_contenere(&self, altro: &Rectangle) -> bool {
        self.larghezza > altro.larghezza && self.altezza > altro.altezza
    }
}
// ANCHOR_END: here

fn main() {
    let rettangolo1 = Rettangolo {
        larghezza: 30,
        altezza: 50,
    };
    let rettangolo2 = Rettangolo {
        larghezza: 10,
        altezza: 40,
    };
    let rettangolo3 = Rettangolo {
        larghezza: 60,
        altezza: 45,
    };

    println!("Può rettangolo1 contenere rettangolo2? {}", rettangolo1.puo_contenere(&rettangolo2));
    println!("Può rettangolo1 contenere rettangolo3? {}", rettangolo1.puo_contenere(&rettangolo3));
}
