pub trait Sommario {
    fn riassunto(&self) -> String {
        String::from("(Leggi di più...)")
    }
}

pub struct Articolo {
    pub titolo: String,
    pub posizione: String,
    pub autore: String,
    pub contenuto: String,
}

impl Sommario for Articolo {}

pub struct PostSocial {
    pub nomeutente: String,
    pub contenuto: String,
    pub risposta: bool,
    pub riposta: bool,
}

impl Sommario for PostSocial {
    fn riassunto(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}
