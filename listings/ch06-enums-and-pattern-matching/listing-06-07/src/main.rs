#[derive(Debug)] // so we can inspect the state in a minute
enum StatoUSA {
    Alabama,
    Alaska,
    // --snip--
}

// ANCHOR: state
impl StatoUSA {
    fn esistente_nel(&self, anno: u16) -> bool {
        match self {
            StatoUSA::Alabama => anno >= 1819,
            StatoUSA::Alaska => anno >= 1959,
            // -- snip --
        }
    }
}
// ANCHOR_END: state

enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter(StatoUSA),
}

// ANCHOR: describe
fn desc_quarter_statale(moneta: Moneta) -> Option<String> {
    if let Moneta::Quarter(stato) = moneta {
        if stato.esistente_nel(1900) {
            Some(format!("{stato:?} è abbastanza vecchio, per l'America!"))
        } else {
            Some(format!("{stato:?} è abbastanza recente."))
        }
    } else {
        None
    }
}
// ANCHOR_END: describe

fn main() {
    if let Some(desc) = desc_quarter_statale(Moneta::Quarter(StatoUSA::Alaska)) {
        println!("{desc}");
    }
}
