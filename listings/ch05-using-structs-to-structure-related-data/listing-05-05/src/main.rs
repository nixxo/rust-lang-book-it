struct Utente {
    attivo: bool,
    nome_utente: String,
    email: String,
    numero_accessi: u64,
}

// ANCHOR: here
fn nuovo_utente(email: String, nome_utente: String) -> Utente {
    Utente {
        attivo: true,
        nome_utente,
        email,
        numero_accessi: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let utente1 = nuovo_utente(
        String::from("qualcuno@mia_mail.com"),
        String::from("qualcuno123"),
    );
}
