struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Chiamami Ishmael. Alcuni anni fa...");
    let prima_frase = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: prima_frase,
    };
}
