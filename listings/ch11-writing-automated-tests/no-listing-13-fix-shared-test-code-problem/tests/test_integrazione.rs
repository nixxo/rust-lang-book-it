use addizione::aggiungi_due;

mod comune;

#[test]
fn aggiungere_due() {
    comune::inizializzazione();

    let risultato = aggiungi_due(2);
    assert_eq!(risultato, 4);
}
