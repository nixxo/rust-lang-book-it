pub trait Sommario {
    fn riassunto(&self) -> String;
}

pub struct Articolo {
    pub titolo: String,
    pub posizione: String,
    pub autore: String,
    pub contenuto: String,
}

impl Sommario for Articolo {
    fn riassunto(&self) -> String {
        format!("{}, di {} ({})", self.titolo, self.autore, self.posizione)
    }
}

pub struct PostSocial {
    pub nomeutente: String,
    pub contenuto: String,
    pub risposta: bool,
    pub repost: bool,
}

impl Sommario for PostSocial {
    fn riassunto(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}

// ANCHOR: here
fn riassumibile(switch: bool) -> impl Sommario {
    if switch {
        Articolo {
            titolo: String::from(
                "I Penguins vincono la Stanley Cup!",
            ),
            posizione: String::from("Pittsburgh, PA, USA"),
            autore: String::from("Iceburgh"),
            contenuto: String::from(
                "I Pittsburgh Penguins sono ancora una volta la migliore squadra di hockey nella NHL.",
            ),
        }
    } else {
        PostSocial {
            nomeutente: String::from("horse_ebooks"),
            contenuto: String::from(
                "ovviamente, come probabilmente già sapete, gente",
            ),
            risposta: false,
            riposta: false,
        }
    }
}
// ANCHOR_END: here
