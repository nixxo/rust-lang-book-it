#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

impl Rettangolo {
    fn area(&self) -> u32 {
        self.larghezza * self.altezza
    }
}

fn main() {
    let rettangolo1 = Rettangolo {
        larghezza: 30,
        altezza: 50,
    };

    println!(
        "L'area del rettangolo è di {} pixel quadrati.",
        rettangolo1 .area()
    );
}
