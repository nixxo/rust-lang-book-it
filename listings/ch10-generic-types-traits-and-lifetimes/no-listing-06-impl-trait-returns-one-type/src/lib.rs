pub trait Sommario {
    fn riassunto(&self) -> String;
}

pub struct ArticoloNews {
    pub titolo: String,
    pub posizione: String,
    pub autore: String,
    pub contenuto: String,
}

impl Sommario for ArticoloNews {
    fn riassunto(&self) -> String {
        format!("{}, di {} ({})", self.titolo, self.autore, self.posizione)
    }
}

pub struct SocialPost {
    pub nomeutente: String,
    pub contenuto: String,
    pub risposta: bool,
    pub riposta: bool,
}

impl Sommario for SocialPost {
    fn riassunto(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}

// ANCHOR: here
fn riassumibile(switch: bool) -> impl Sommario {
    if switch {
        ArticoloNews {
            titolo: String::from(
                "I pinguini vincono il campionato di Stanley Cup!",
            ),
            posizione: String::from("Pittsburgh, PA, USA"),
            autore: String::from("Iceburgh"),
            contenuto: String::from(
                "I Pittsburgh Penguins sono ancora una volta la migliore squadra di hockey nella NHL.",
            ),
        }
    } else {
        SocialPost {
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
