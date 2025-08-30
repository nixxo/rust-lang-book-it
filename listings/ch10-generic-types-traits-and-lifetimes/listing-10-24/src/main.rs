struct ParteImportante<'a> {
    parte: &'a str,
}

fn main() {
    let romanzo = String::from("Chiamami Ishmael. Alcuni anni fa...");
    let prima_frase = romanzo.split('.').next().unwrap();
    let i = ParteImportante {
        parte: prima_frase,
    };
}
