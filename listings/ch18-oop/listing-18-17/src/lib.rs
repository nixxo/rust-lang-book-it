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

    // ANCHOR: here
    pub fn contenuto(&self) -> &str {
        self.stato.as_ref().unwrap().contenuto(self)
    }
    // --taglio--
    // ANCHOR_END: here

    pub fn richiedi_revisione(&mut self) {
        if let Some(s) = self.stato.take() {
            self.stato = Some(s.richiedi_revisione())
        }
    }

    pub fn approva(&mut self) {
        if let Some(s) = self.stato.take() {
            self.stato = Some(s.approva())
        }
    }
    // ANCHOR: here
}
// ANCHOR_END: here

trait Stato {
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato>;
    fn approva(self: Box<Self>) -> Box<dyn Stato>;
}

struct Bozza {}

impl Stato for Bozza {
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        Box::new(AttesaRevisione {})
    }

    fn approva(self: Box<Self>) -> Box<dyn Stato> {
        self
    }
}

struct AttesaRevisione {}

impl Stato for AttesaRevisione {
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        self
    }

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
