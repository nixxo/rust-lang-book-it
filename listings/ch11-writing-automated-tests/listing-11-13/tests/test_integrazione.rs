use addizione::aggiungi_due;

#[test]
fn aggiungere_due() {
    let risultato = aggiungi_due(2);
    assert_eq!(risultato, 4);
}
