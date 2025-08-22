#[derive(Debug)] // so we can inspect the state in a minute
enum StatoUSA {
    Alabama,
    Alaska,
    // --snip--
}

impl StatoUSA {
    fn esistente_nel(&self, anno: u16) -> bool {
        match self {
            StatoUSA::Alabama => anno >= 1819,
            StatoUSA::Alaska => anno >= 1959,
            // -- snip --
        }
    }
}

enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter(StatoUSA),
}

// ANCHOR: describe
fn desc_quarter_statale(moneta: Moneta) -> Option<String> {
    let Moneta::Quarter(stato) = moneta else {
        return None;
    };

    if stato.esistente_nel(1900) {
        Some(format!("{stato:?} è abbastanza vecchio, per l'America!"))
    } else {
        Some(format!("{stato:?} è abbastanza recente."))
    }
}
// ANCHOR_END: describe

fn main() {
    if let Some(desc) = desc_quarter_statale(Moneta::Quarter(StatoUSA::Alaska)) {
        println!("{desc}");
    }
}
