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
    pub riposta: bool,
}

impl Sommario for PostSocial {
    fn riassunto(&self) -> String {
        format!("{}: {}", self.nomeutente, self.contenuto)
    }
}
