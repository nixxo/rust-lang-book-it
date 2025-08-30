pub fn saluto(nome: &str) -> String {
    String::from("Ciao!")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn saluto_contiene_nome() {
        let risultato = saluto("Carol");
        assert!(
            risultato.contains("Carol"),
            "Saluto non contiene un nome, il valore era `{risultato}`"
        );
    }
    // ANCHOR_END: here
}
