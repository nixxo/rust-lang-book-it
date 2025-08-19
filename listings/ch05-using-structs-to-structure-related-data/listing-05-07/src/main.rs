struct Utente {
    attivo: bool,
    nome_utente: String,
    email: String,
    numero_accessi: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let utente1 = Utente {
        email: String::from("qualcuno@mia_mail.com"),
        nome_utente: String::from("qualcuno123"),
        attivo: true,
        numero_accessi: 1,
    };
    // ANCHOR: here

    let utente2 = Utente {
        email: String::from("altra_mail@example.com"),
        ..utente1
    };
}
// ANCHOR_END: here
