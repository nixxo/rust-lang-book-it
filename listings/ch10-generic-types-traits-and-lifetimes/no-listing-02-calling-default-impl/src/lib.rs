pub trait Sommario {
    fn riassunto(&self) -> String {
        String::from("(Leggi di più...)")
    }
}

pub struct ArticoloNews {
    pub titolo: String,
    pub posizione: String,
    pub autore: String,
    pub contenuto: String,
}

impl Sommario for ArticoloNews {}

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
