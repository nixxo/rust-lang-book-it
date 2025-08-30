struct ParteImportante<'a> {
    parte: &'a str,
}

// ANCHOR: 1st
impl<'a> ParteImportante<'a> {
    fn livello(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> ParteImportante<'a> {
    fn annunciare_e_restituire_parte(&self, annuncio: &str) -> &str {
        println!("Attenzione per favore: {annuncio}");
        self.parte
    }
}
// ANCHOR_END: 3rd

fn main() {
    let romanzo = String::from("Chiamami Ishmael. Qualche anno fa...");
    let prima_frase = romanzo.split('.').next().unwrap();
    let i = ParteImportante {
        parte: prima_frase,
    };
}
