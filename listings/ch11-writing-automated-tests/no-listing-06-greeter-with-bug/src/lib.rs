// ANCHOR: here
pub fn saluto(nome: &str) -> String {
    String::from("Ciao!")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saluto_contiene_nome() {
        let risultato = saluto("Carol");
        assert!(risultato.contains("Carol"));
    }
}
