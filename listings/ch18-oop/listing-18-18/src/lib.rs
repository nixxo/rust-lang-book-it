pub struct Post {
    stato: Option<Box<dyn Stato>>,
    contenuto: String,
}

impl Post {
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
        self.stato.as_ref().unwrap().contenuto(self)
    }

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
}

// ANCHOR: here
trait Stato {
    // --taglio--
    // ANCHOR_END: here
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato>;
    fn approva(self: Box<Self>) -> Box<dyn Stato>;

    // ANCHOR: here
    fn contenuto<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --taglio--
// ANCHOR_END: here

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

// ANCHOR: here
struct Pubblicato {}

impl Stato for Pubblicato {
    // --taglio--
    // ANCHOR_END: here
    fn richiedi_revisione(self: Box<Self>) -> Box<dyn Stato> {
        self
    }

    fn approva(self: Box<Self>) -> Box<dyn Stato> {
        self
    }

    // ANCHOR: here
    fn contenuto<'a>(&self, post: &'a Post) -> &'a str {
        &post.contenuto
    }
}
// ANCHOR_END: here
