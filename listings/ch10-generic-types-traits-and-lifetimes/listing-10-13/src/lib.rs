pub trait Sommario {
    fn sommarizza(&self) -> String;
}

// ANCHOR: here
pub struct ArticoloNews {
    pub titolo: String,
    pub posizione: String,
    pub autore: String,
    pub contenuto: String,
}

impl Sommario for ArticoloNews {
    fn sommarizza(&self) -> String {
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
    fn sommarizza(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}
// ANCHOR_END: here
