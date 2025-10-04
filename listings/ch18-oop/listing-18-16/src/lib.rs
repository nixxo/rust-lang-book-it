pub struct Post {
    stato: Option<Box<dyn Stato>>,
    contenuto: String,
}

// ANCHOR: here
impl Post {
    // --taglio--
    // ANCHOR_END: here
    pub fn new() -> Post {
        Post {
            stato: Some(Box::new(Bozza {})),
            contenuto: String::new(),
        }
    }

    pub fn aggiungi_testo(&mut self, testo: &str) {
        self.contenuto.push_str(testo);
    }

    pub fn contenuto(&self) -> &str {
        ""
    }

    pub fn richiedi_revisione(&mut self) {
        if let Some(s) = self.stato.take() {
            self.stato = Some(s.richiedi_revisione())
        }
    }

    // ANCHOR: here
    pub fn approva(&mut self) {
        if let Some(s) = self.stato.take() {
            self.stato = Some(s.approva())
        }
    }
}

trait Stato {
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato>;
    fn approva(self: Box<Self>) -> Box<dyn Stato>;
}

struct Bozza {}

impl Stato for Bozza {
    // --taglio--
    // ANCHOR_END: here
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        Box::new(AttesaRevisione {})
    }

    // ANCHOR: here
    fn approva(self: Box<Self>) -> Box<dyn Stato> {
        self
    }
}

struct AttesaRevisione {}

impl Stato for AttesaRevisione {
    // --taglio--
    // ANCHOR_END: here
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        self
    }

    // ANCHOR: here
    fn approva(self: Box<Self>) -> Box<dyn Stato> {
        Box::new(Pubblicato {})
    }
}

struct Pubblicato {}

impl Stato for Pubblicato {
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        self
    }

    fn approva(self: Box<Self>) -> Box<dyn Stato> {
        self
    }
}
// ANCHOR_END: here
