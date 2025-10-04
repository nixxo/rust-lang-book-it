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

    // ANCHOR: here
    pub fn aggiungi_testo(&mut self, testo: &str) {
        self.contenuto.push_str(testo);
    }
}
// ANCHOR_END: here

trait Stato {}

struct Bozza {}

impl Stato for Bozza {}
