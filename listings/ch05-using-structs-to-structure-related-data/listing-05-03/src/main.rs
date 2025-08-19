struct Utente {
    attivo: bool,
    nome_utente: String,
    email: String,
    numero_accessi: u64,
}

// ANCHOR: here
fn main() {
    let mut utente1 = Utente {
        attivo: true,
        nome_utente: String::from("qualcuno123"),
        email: String::from("qualcuno@mia_mail.com"),
        numero_accessi: 1,
    };

    user1.email = String::from("nuova_email@mia_mail.com");
}
// ANCHOR_END: here
