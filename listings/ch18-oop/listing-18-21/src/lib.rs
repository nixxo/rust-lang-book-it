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

impl PostBozza {
    pub fn aggiungi_testo(&mut self, testo: &str) {
        self.contenuto.push_str(testo);
    }

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
