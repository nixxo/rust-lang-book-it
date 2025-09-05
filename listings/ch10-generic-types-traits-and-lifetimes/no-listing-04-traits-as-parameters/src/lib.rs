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
    pub repost: bool,
}

impl Sommario for SocialPost {
    fn riassunto(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}

// ANCHOR: here
pub fn notifica(elemento: &impl Sommario) {
    println!("Ultime notizie! {}", elemento.riassunto());
}
// ANCHOR_END: here
