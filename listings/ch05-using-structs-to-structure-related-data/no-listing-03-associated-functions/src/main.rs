#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

// ANCHOR: here
impl Rettangolo {
    fn quadrato(dimensione: u32) -> Self {
        Self {
            larghezza: dimensione,
            altezza: dimensione,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let quad = Rectangle::quadrato(3);
}
