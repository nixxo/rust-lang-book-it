// ANCHOR: here
pub trait Sommario {
    fn riassunto_autore(&self) -> String;

    fn riassunto(&self) -> String {
        format!("(Leggi di più da {}...)", self.riassunto_autore())
    }
}
// ANCHOR_END: here

pub struct PostSocial {
    pub nomeutente: String,
    pub contenuto: String,
    pub risposta: bool,
    pub repost: bool,
}

// ANCHOR: impl
impl Sommario for PostSocial {
    fn riassunto_autore(&self) -> String {
        format!("@{}", self.nomeutente)
    }
}
// ANCHOR_END: impl
