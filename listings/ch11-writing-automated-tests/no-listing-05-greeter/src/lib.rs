pub fn saluto(nome: &str) -> String {
    format!("Ciao {nome}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saluto_contiene_nome() {
        let risultato = saluto("Carol");
        assert!(risultato.contains("Carol"));
    }
}
