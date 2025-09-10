mod cucine {
    pub struct Colazione {
        pub toast: String,
        frutta_di_stagione: String,
    }

    impl Colazione {
        pub fn estate(toast: &str) -> Colazione {
            Colazione {
                toast: String::from(toast),
                frutta_di_stagione: String::from("pesche"),
            }
        }
    }
}

pub fn mangiare_al_ristorante() {
    // Ordina una colazione in estate con pane tostato di segale.
    let mut pasto = cucine::Colazione::estate("segale");
    // Cambiare idea sul pane che vorremmo.
    pasto.toast = String::from("integrale");
    println!("Vorrei un toast {}, grazie.", pasto.toast);

    // La riga successiva non verrà compilata se la de-commentiamo; non
    // ci è permesso vedere o modificare frutta che accompagna il pasto.
    // pasto.frutta_di_stagione = String::from("mirtilli");
}
