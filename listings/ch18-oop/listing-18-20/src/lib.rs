pub struct Post {
    contenuto: String,
}

pub struct PostBozza {
    contenuto: String,
}

impl Post {
    pub fn new() -> PostBozza {
        PostBozza {
            contenuto: String::new(),
        }
    }

    pub fn contenuto(&self) -> &str {
        &self.contenuto
    }
}

// ANCHOR: here
impl PostBozza {
    // --taglio--
    // ANCHOR_END: here
    pub fn aggiungi_testo(&mut self, testo: &str) {
        self.contenuto.push_str(testo);
    }

    // ANCHOR: here
    pub fn richiedi_revisione(self) -> PostAttesaRevisione {
        PostAttesaRevisione {
            contenuto: self.contenuto,
        }
    }
}

pub struct PostAttesaRevisione {
    contenuto: String,
}

impl PostAttesaRevisione {
    pub fn approva(self) -> Post {
        Post {
            contenuto: self.contenuto,
        }
    }
}
// ANCHOR_END: here
